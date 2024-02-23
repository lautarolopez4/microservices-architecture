// Importing necessary modules and types from Rust standard library and external crates
pub mod auth;
pub mod error;

use auth::{with_auth, Role};
use error::Error::*; // Importing functions and types from the 'auth' module

// Importing necessary types and traits from Rust standard library and external crates
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{
    reject,
    reply::{self, Json},
    Filter, Rejection, Reply,
};

// Defining type aliases for Result and WebResult for convenience
type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
type Users = Arc<HashMap<String, User>>;

// Defining the structure representing a user
#[derive(Clone)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub pass: String,
    pub role: String,
    pub otp_secret: Option<String>,
}

// Defining the structure representing a login request
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pw: String,
}

// Defining the structure representing a login response
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

// Main function to run the application
#[tokio::main]
async fn main() {
    // Initializing the users hashmap and wrapping it in an Arc to share across threads
    let users = Arc::new(init_users());

    // Defining routes using filters provided by the 'warp' crate
    let login_route = warp::path!("login")
        .and(warp::post())
        .and(with_users(users.clone()))
        .and(warp::body::json())
        .and_then(login_handler);

    let user_route = warp::path!("user")
        .and(with_auth(Role::User))
        .and_then(user_handler);
    let admin_route = warp::path!("admin")
        .and(with_auth(Role::Admin))
        .and_then(admin_handler);

    let users_list_route = warp::path!("admin" / "users")
        .and(warp::get())
        .and(with_users(users.clone()))
        .and(with_auth(Role::Admin))
        .and_then(admin_users_handler);

    // Combining routes and handling potential errors using 'recover' method
    let routes = login_route
        .or(user_route)
        .or(admin_route)
        .or(users_list_route)
        .recover(error::handle_rejection);

    // Starting the Warp server to serve the defined routes
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

// Function to create a filter to inject the users hashmap into request handlers
fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}

// Request handler to handle login requests
pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply> {
    match users
        .iter()
        .find(|(_uid, user)| user.email == body.email && user.pass == body.pw)
    {
        Some((uid, user)) => {
            let token = auth::create_jwt(&uid, &Role::from_str(&user.role))
                .map_err(|e| reject::custom(e))?;
            Ok(reply::json(&LoginResponse { token }))
        }
        None => Err(reject::custom(WrongCredentialsError)),
    }
}

// Request handler to handle user requests
pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello Users {}", uid))
}

// Request handler to handle admin requests
pub async fn admin_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello Admin {}", uid))
}

async fn admin_users_handler(users: Users, uid: String) -> WebResult<impl Reply> {
    let user_list: Vec<&User> = users.values().collect();

    let user_data: Vec<serde_json::Value> = user_list
        .iter()
        .map(|user| {
            json!({
                "uid": &user.uid,
                "email": &user.email,
                "role": &user.role,
            })
        })
        .collect();

    Ok(reply::json(&user_data))
}

// Function to initialize the users hashmap
fn init_users() -> HashMap<String, User> {
    let mut map = HashMap::new();
    map.insert(
        String::from("1"),
        User {
            uid: String::from("1"),
            email: String::from("user@testuser.com"),
            pass: String::from("1234"),
            role: String::from("User"),
            otp_secret: None,
        },
    );

    map.insert(
        String::from("2"),
        User {
            uid: String::from("2"),
            email: String::from("admin@adminland.com"),
            pass: String::from("4321"),
            role: String::from("Admin"),
            otp_secret: None,
        },
    );
    map
}
