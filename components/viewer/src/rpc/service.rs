use thiserror::Error;

use crate::manage_objects::global::INTERNAL_REQUEST_LIST;
use crate::manage_objects::request::{self, InternalRequest, object::ObjectRequest};
use crate::rpc::proto::generated::ObjectSize;

use super::proto::generated::manage_object_service_server::ManageObjectService;
use super::proto::generated::{
    ObjectColor, ObjectColorEnum, ObjectId, ObjectShape, SetObjectPositionRequest,
    SetObjectPositionResponse, SetObjectPositionSequenceRequest, SetObjectPositionSequenceResponse,
    SpawnObjectRequest, SpawnObjectResponse, SpawnObjectSequenceRequest,
    SpawnObjectSequenceResponse, Uuid, object_color,
};

use bevy::log::{trace, trace_span, warn};

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
        // Create a span for tracing
        let _span = trace_span!("set_object_position_rpc").entered();

        let request = request.into_inner();

        let internal_request = match set_position_request_to_internal_request(request) {
            Ok(object) => object,
            Err(e) => {
                return match e {
                    SetObjectPositionError::InvalidObjectId => {
                        Err(tonic::Status::invalid_argument(e.to_string()))
                    }
                    SetObjectPositionError::InvalidPosition => {
                        Err(tonic::Status::invalid_argument(e.to_string()))
                    }
                };
            }
        };

        trace!("Internal request: {:?}", internal_request);

        INTERNAL_REQUEST_LIST.push(InternalRequest::ObjectRequest(ObjectRequest::SetPosition(
            internal_request.clone(),
        )));

        trace!("Set position request added to queue");

        Ok(Response::new(SetObjectPositionResponse { success: true }))
    }

    #[doc = " Spawns a new object in the scene."]
    async fn spawn_object(
        &self,
        request: tonic::Request<SpawnObjectRequest>,
    ) -> std::result::Result<tonic::Response<SpawnObjectResponse>, tonic::Status> {
        let _span = trace_span!("spawn_object_rpc").entered();

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

        trace!("Internal request: {:?}", internal_request);

        INTERNAL_REQUEST_LIST.push(InternalRequest::ObjectRequest(ObjectRequest::Spawn(
            internal_request.clone(),
        )));

        trace!("Spawn request added to queue");

        Ok(Response::new(SpawnObjectResponse {
            spawend_object_id: Some(ObjectId {
                uuid: Some(Uuid {
                    value: internal_request.object_id.uuid.as_bytes().to_vec(),
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
        let request = request.into_inner();
        let SetObjectPositionSequenceRequest { requests } = request;

        let mut set_object_responses: Vec<SetObjectPositionResponse> = Vec::new();

        for (index, request) in requests.into_iter().enumerate() {
            let response = self
                .set_object_position(tonic::Request::new(request))
                .await
                .map_err(|e| {
                    let errror_message = e.message();
                    tonic::Status::new(e.code(), format!("Index {index}; {errror_message}"))
                })?;

            set_object_responses.push(response.into_inner());
        }

        Ok(Response::new(SetObjectPositionSequenceResponse {
            responses: set_object_responses,
        }))
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
                    let errror_message = e.message();
                    tonic::Status::new(e.code(), format!("Index {index}; {errror_message}"))
                })?;

            spawn_object_responses.push(response.into_inner());
        }

        Ok(Response::new(SpawnObjectSequenceResponse {
            responses: spawn_object_responses,
        }))
    }
}

#[derive(Error, Debug)]
pub enum SetObjectPositionError {
    #[error("Invalid object ID")]
    InvalidObjectId,
    #[error("Invalid position")]
    InvalidPosition,
}

/// Converts a gRPC SetObjectPositionRequest into an internal request, validating fields.
pub fn set_position_request_to_internal_request(
    set_position_request: SetObjectPositionRequest,
) -> std::result::Result<request::object::SetObjectPositionRequest, SetObjectPositionError> {
    let SetObjectPositionRequest {
        object_id,
        position,
    } = set_position_request;

    trace!(
        "Received request to set object position {:?} to {:?}",
        object_id, position
    );

    let object_id = object_id.ok_or(SetObjectPositionError::InvalidObjectId)?;
    let uuid = object_id
        .uuid
        .ok_or(SetObjectPositionError::InvalidObjectId)?;

    let position = position.ok_or(SetObjectPositionError::InvalidPosition)?;

    let internal_request = request::object::SetObjectPositionRequest {
        object_id: request::object::ObjectId {
            uuid: uuid::Uuid::from_slice(uuid.value.as_slice())
                .map_err(|_| SetObjectPositionError::InvalidObjectId)?,
        },
        position: Vec3::new(position.x, position.y, position.z),
    };

    Ok(internal_request)
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

/// Converts a gRPC SpawnObjectRequest into an internal request, validating fields and assigning a UUID.
pub fn spawn_object_request_to_internal_request(
    spawn_object_request: SpawnObjectRequest,
) -> std::result::Result<request::object::SpawnObjectRequest, SpawnObjectError> {
    let SpawnObjectRequest {
        object_properties,
        position,
    } = spawn_object_request;

    trace!(
        "Received request to spawn object {:?} at position {:?}",
        object_properties, position
    );

    let position = position.ok_or(SpawnObjectError::InvalidPosition)?;

    let object_properties = object_properties.ok_or(SpawnObjectError::InvalidObjectProperties)?;

    let object_color = object_properties
        .color
        .ok_or(SpawnObjectError::InvalidObjectColor)?;

    let object_size = object_properties.size.unwrap_or(ObjectSize { value: 1.0 });

    let bevy_color =
        normalize_object_color(object_color).map_err(|_| SpawnObjectError::InvalidObjectColor)?;

    let spawn_object_uuid = uuid::Uuid::now_v7();

    let spawn_request = request::object::SpawnObjectRequest {
        object_id: request::object::ObjectId {
            uuid: spawn_object_uuid,
        },
        object_properties: request::object::ObjectProperties {
            color: bevy_color,
            shape: match ObjectShape::try_from(object_properties.shape) {
                Ok(ObjectShape::Cube) => request::object::ObjectShape::Cube,
                Ok(ObjectShape::Sphere) => request::object::ObjectShape::Sphere,
                _ => return Err(SpawnObjectError::InvalidObjectShape),
            },
            size: object_size.value,
        },
        position: Vec3::new(position.x, position.y, position.z),
    };

    Ok(spawn_request)
}

/// Transforms a gRPC ObjectColor into a Bevy Color, validating values and enum variants.
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
            trace!("Object color is RGBA: {:?}", rgba);
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
