use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    // -- Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailContextNotInRequestExtension,

    // -- Model Errors
    TicketDeleteFailIdNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("-->> {:<20} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAILED),

            // Auth
            Self::AuthFailContextNotInRequestExtension
            | Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // Model.
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::NOT_FOUND, ClientError::NO_AUTH)
            }

            // Fallback Service
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, strum_macros::AsRefStr)]
pub enum ClientError {
    LOGIN_FAILED,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
