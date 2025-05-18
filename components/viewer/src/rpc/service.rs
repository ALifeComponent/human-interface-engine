use std::time::SystemTime;

use crate::manage_objects;
use crate::manage_objects::global::SPAWN_OBJECT_REQUEST_LIST;
use crate::rpc::proto::generated::{Object, ObjectColorEnum, ObjectId, ObjectProperties, Rgba};

use super::proto::generated::manage_object_service_server::ManageObjectService;
use super::proto::generated::object_color::Color;
use super::proto::generated::{
    ObjectColor, ObjectType, SetObjectPositionRequest, SetObjectPositionResponse,
    SpawnObjectRequest, SpawnObjectResponse, Uuid,
};

use bevy::log::{info, warn};

use bevy::math::Vec3;
use prost::Message;
use tonic::Response;

#[derive(Default)]
pub struct ManageObjectServiceImpl {}

#[tonic::async_trait]
impl ManageObjectService for ManageObjectServiceImpl {
    #[doc = " Sets the position of the object."]
    async fn set_object_position(
        &self,
        request: tonic::Request<SetObjectPositionRequest>,
    ) -> std::result::Result<tonic::Response<SetObjectPositionResponse>, tonic::Status> {
        let request = request.into_inner();
        let SetObjectPositionRequest {
            object_id,
            position,
        } = request;

        info!(
            "Received request to set position of object {:?} to {:?}",
            object_id, position
        );

        return Ok(Response::new(SetObjectPositionResponse { success: true }));
    }

    #[doc = " Spawns a new object in the scene."]
    async fn spawn_object(
        &self,
        request: tonic::Request<SpawnObjectRequest>,
    ) -> std::result::Result<tonic::Response<SpawnObjectResponse>, tonic::Status> {
        let request = request.into_inner();
        let SpawnObjectRequest {
            object_properties,
            position,
        } = request;

        info!(
            "Received request to spawn object {:?} at position {:?}",
            object_properties, position
        );

        if position.is_none() {
            return Err(tonic::Status::invalid_argument("Position is None"));
        }
        let position = position.unwrap();

        // Validate the object properties
        if object_properties.is_none() {
            return Err(tonic::Status::invalid_argument("Object color is None"));
        }
        let object_properties = object_properties.unwrap();

        if object_properties.color.is_none() {
            return Err(tonic::Status::invalid_argument("Object color is None"));
        }

        let bevy_color: bevy::color::Color;
        if let Ok(color) = normalize_object_color(object_properties.color.clone().unwrap()) {
            bevy_color = color;
            info!("Normalized object color: {:?}", color);
        } else {
            return Err(tonic::Status::invalid_argument("Invalid object color"));
        }

        let spawn_object_request = manage_objects::SpawnObjectRequest {
            object_id: manage_objects::ObjectId {
                uuid: uuid::Uuid::now_v7(),
            },
            object_properties: manage_objects::ObjectProperties {
                color: bevy_color,
                shape: match ObjectType::try_from(object_properties.r#type) {
                    Ok(ObjectType::Cube) => manage_objects::ObjectShape::Cube,
                    Ok(ObjectType::Sphere) => manage_objects::ObjectShape::Sphere,
                    _ => return Err(tonic::Status::invalid_argument("Invalid object shape")),
                },
                size: 1.0, // TODO: Set size based on object properties
            },
            position: Vec3::new(position.x, position.y, position.z),
        };

        let bevy_color_srgb = bevy_color.to_srgba();

        let spawned_object = Object {
            id: Some(ObjectId {
                uuid: Some(Uuid {
                    uuid: spawn_object_request
                        .object_id
                        .uuid
                        .clone()
                        .as_bytes()
                        .to_vec(),
                }),
            }),
            properties: Some(ObjectProperties {
                r#type: object_properties.r#type,
                color: Some(ObjectColor {
                    color: Some(Color::ColorRgba(Rgba {
                        r: bevy_color_srgb.red,
                        g: bevy_color_srgb.green,
                        b: bevy_color_srgb.blue,
                        a: bevy_color_srgb.alpha,
                    })),
                }),
            }),
        };

        // Add the spawn request to the queue
        SPAWN_OBJECT_REQUEST_LIST.queue.push(spawn_object_request);

        info!("Spawn request added to queue");

        Ok(Response::new(SpawnObjectResponse {
            spawend_object: Some(spawned_object),
        }))
    }
}

pub fn normalize_object_color(object_color: ObjectColor) -> anyhow::Result<bevy::color::Color> {
    let ObjectColor { color } = object_color;

    if color.is_none() {
        return Err(anyhow::anyhow!("Object color is None"));
    }
    let color: Color = color.unwrap();

    let bevy_color = match color {
        Color::ColorEnum(n) => {
            let color = ObjectColorEnum::try_from(n)?;
            match color {
                ObjectColorEnum::Blue => bevy::color::Color::srgb(0.0, 0.0, 1.0),
                ObjectColorEnum::Green => bevy::color::Color::srgb(0.0, 1.0, 0.0),
                ObjectColorEnum::Red => bevy::color::Color::srgb(1.0, 0.0, 0.0),
                ObjectColorEnum::Unspecified => {
                    warn!("Object color is unspecified");
                    bevy::color::Color::srgb(1.0, 1.0, 1.0)
                }
            }
        }
        Color::ColorRgba(rgba) => {
            info!("Object color is RGBA: {:?}", rgba);
            if (rgba.r > 1.0 || rgba.g > 1.0 || rgba.b > 1.0 || rgba.a > 1.0)
                || (rgba.r < 0.0 || rgba.g < 0.0 || rgba.b < 0.0 || rgba.a < 0.0)
            {
                return Err(anyhow::anyhow!(
                    "Object color rgba values must be between 0.0 and 1.0"
                ));
            }

            bevy::color::Color::srgba(rgba.r, rgba.g, rgba.b, rgba.a)
        }
    };

    Ok(bevy_color)
}
