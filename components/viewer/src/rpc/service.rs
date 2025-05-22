use thiserror::Error;

use crate::manage_objects;
use crate::manage_objects::global::SPAWN_OBJECT_REQUEST_LIST;

use super::proto::generated::manage_object_service_server::ManageObjectService;
use super::proto::generated::{
    ObjectColor, ObjectColorEnum, ObjectId, ObjectShape, SetObjectPositionRequest,
    SetObjectPositionResponse, SetObjectPositionSequenceRequest, SetObjectPositionSequenceResponse,
    SpawnObjectRequest, SpawnObjectResponse, SpawnObjectSequenceRequest,
    SpawnObjectSequenceResponse, Uuid, object_color,
};

use bevy::log::{info, warn};

use bevy::math::Vec3;
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

        todo!()
    }

    #[doc = " Spawns a new object in the scene."]
    async fn spawn_object(
        &self,
        request: tonic::Request<SpawnObjectRequest>,
    ) -> std::result::Result<tonic::Response<SpawnObjectResponse>, tonic::Status> {
        let request = request.into_inner();

        let internal_request = match spawn_object_request_to_internal_request(request) {
            Ok(object) => object,
            Err(e) => {
                return match e {
                    SpawnObjectError::InvalidObjectColor => {
                        Err(tonic::Status::invalid_argument(e.to_string()))
                    }
                    SpawnObjectError::InvalidObjectShape => {
                        Err(tonic::Status::invalid_argument(e.to_string()))
                    }
                    SpawnObjectError::InvalidPosition => {
                        Err(tonic::Status::invalid_argument(e.to_string()))
                    }
                    SpawnObjectError::InvalidObjectProperties => {
                        Err(tonic::Status::invalid_argument(e.to_string()))
                    }
                };
            }
        };

        info!("Internal request: {:?}", internal_request);

        SPAWN_OBJECT_REQUEST_LIST.push(internal_request.clone());

        info!("Spawn request added to queue");

        Ok(Response::new(SpawnObjectResponse {
            spawend_object_id: Some(ObjectId {
                uuid: Some(Uuid {
                    uuid: internal_request.object_id.uuid.as_bytes().to_vec(),
                }),
            }),
        }))
    }

    #[doc = " Sets the position of multiple objects in a single request."]
    async fn set_object_position_sequence(
        &self,
        request: tonic::Request<SetObjectPositionSequenceRequest>,
    ) -> std::result::Result<tonic::Response<SetObjectPositionSequenceResponse>, tonic::Status>
    {
        todo!()
    }

    #[doc = " Spawns multiple objects in a single request."]
    async fn spawn_object_sequence(
        &self,
        request: tonic::Request<SpawnObjectSequenceRequest>,
    ) -> std::result::Result<tonic::Response<SpawnObjectSequenceResponse>, tonic::Status> {
        let request = request.into_inner();
        let SpawnObjectSequenceRequest { requests } = request;

        let mut spawn_object_responses: Vec<SpawnObjectResponse> = Vec::new();

        for (index, request) in requests.into_iter().enumerate() {
            let response = self
                .spawn_object(tonic::Request::new(request))
                .await
                .map_err(|e| {
                    tonic::Status::new(e.code(), format!("Index {index}; {}", e.message()))
                })?;

            spawn_object_responses.push(response.into_inner());
        }

        Ok(Response::new(SpawnObjectSequenceResponse {
            responses: spawn_object_responses,
        }))
    }
}

#[derive(Error, Debug)]
pub enum SpawnObjectError {
    #[error("Invalid object color")]
    InvalidObjectColor,
    #[error("Invalid object shape")]
    InvalidObjectShape,
    #[error("Invalid position")]
    InvalidPosition,
    #[error("Invalid object properties")]
    InvalidObjectProperties,
}

pub fn spawn_object_request_to_internal_request(
    spawn_object_request: SpawnObjectRequest,
) -> std::result::Result<manage_objects::SpawnObjectRequest, SpawnObjectError> {
    let SpawnObjectRequest {
        object_properties,
        position,
    } = spawn_object_request;

    info!(
        "Received request to spawn object {:?} at position {:?}",
        object_properties, position
    );

    let position = position.ok_or(SpawnObjectError::InvalidPosition)?;

    let object_properties = object_properties.ok_or(SpawnObjectError::InvalidObjectProperties)?;

    let object_color = object_properties
        .color
        .ok_or(SpawnObjectError::InvalidObjectColor)?;

    let object_size = object_properties.size.unwrap_or_default();

    let bevy_color =
        normalize_object_color(object_color).map_err(|_| SpawnObjectError::InvalidObjectColor)?;

    let spawn_object_uuid = uuid::Uuid::now_v7();

    Ok(manage_objects::SpawnObjectRequest {
        object_id: manage_objects::ObjectId {
            uuid: spawn_object_uuid,
        },
        object_properties: manage_objects::ObjectProperties {
            color: bevy_color,
            shape: match ObjectShape::try_from(object_properties.r#type) {
                Ok(ObjectShape::Cube) => manage_objects::ObjectShape::Cube,
                Ok(ObjectShape::Sphere) => manage_objects::ObjectShape::Sphere,
                _ => return Err(SpawnObjectError::InvalidObjectShape),
            },
            size: object_size.value,
        },
        position: Vec3::new(position.x, position.y, position.z),
    })
}

pub fn normalize_object_color(object_color: ObjectColor) -> anyhow::Result<bevy::color::Color> {
    let ObjectColor { color } = object_color;

    let color = color.ok_or_else(|| anyhow::anyhow!("Object color is None"))?;

    let bevy_color = match color {
        object_color::Color::ColorEnum(n) => {
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
        object_color::Color::ColorRgba(rgba) => {
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
