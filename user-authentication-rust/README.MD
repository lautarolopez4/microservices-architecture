# Rust Authentication JWT 

Welcome to Rust Authentication - JWT! This project is a simple authentication service built with Rust and Warp.

## Features
- User authentication using JSON Web Tokens (JWT)
- Role-based access control (RBAC) for user authorization

## Installation
1. Ensure you have Rust installed on your system. You can install Rust from [rustup.rs](https://rustup.rs/).
2. Clone this repository to your local machine.
3. Navigate to the project directory.
4. Run `cargo build` to build the project.
5. Run `cargo run` to start the server.

## Usage
### Authentication
To authenticate a user and obtain a JWT token, send a POST request to `/login` with the user's credentials in the request body. Example:

```json
{
  "email": "user@example.com",
  "pw": "password123"
}
```
## User Routes
- `POST /login`: Authenticate a user and obtain a JWT token.
- `GET /user`: Retrieve user information.
- `GET /admin`: Retrieve admin information.

These routes facilitate interaction with user and admin data in the system. The `POST /login` endpoint allows users to authenticate and obtain a JWT token for accessing protected routes. The `GET /user` endpoint retrieves user information, while the `GET /admin` endpoint retrieves admin information.

### Example Usage

#### Authenticate User
```bash
curl -X POST -d '{"email": "user@example.com", "pw": "password123"}' http://localhost:8000/login
```
#### Retrieve User Information
```bash
curl -X GET -H "Authorization: Bearer <TOKEN>" http://localhost:8000/user
```
#### Retrieve Admin Information
```bash
curl -X GET -H "Authorization: Bearer <TOKEN>" http://localhost:8000/admin
```
## Contributing
Pull requests are welcome. However, please note that this project is intended for educational purposes, and there may not be a compelling reason for further expansion or modification. If you still wish to contribute, feel free to open a pull request. For major changes, please open an issue first to discuss what you would like to change.

## License
This project is licensed under the MIT License.
