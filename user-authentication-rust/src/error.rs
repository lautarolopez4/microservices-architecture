// Importing necessary external crates and traits
use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

// Defining custom error types using the `thiserror` crate
#[derive(Debug, Error)]
pub enum Error {
    #[error("Wrong credentials")]
    WrongCredentialsError,
    #[error("JWT token - Creation Error")]
    JWTTokenCreationError,
    #[error("JWT token not valid")]
    JWTTokenError,
    #[error("No auth header")]
    NoAuthHeaderError,
    #[error("No permission")]
    NoPermisionError,
    #[error("Invalid auth error!")]
    InvalidAuthHeaderError,
}

// Structure representing an error response
#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}

// Implementing the `Reject` trait from Warp for custom error handling
impl warp::reject::Reject for Error {}

// Function to handle rejections and convert them into appropriate HTTP responses
pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    // Matching different types of errors and generating appropriate responses
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::WrongCredentialsError => (StatusCode::FORBIDDEN, e.to_string()),
            Error::NoPermisionError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTTokenError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTTokenCreationError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
            _ => (StatusCode::BAD_REQUEST, e.to_string()),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (StatusCode::METHOD_NOT_ALLOWED, "Method not allowed!".to_string())
    } else {
        eprintln!("unhandled error!: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server Error".to_string())
    };

    // Creating JSON response body
    let json = warp::reply::json(&ErrorResponse {
        status: code.to_string(),
        message,
    });
    // Returning the response with appropriate status code
    Ok(warp::reply::with_status(json, code))
}
