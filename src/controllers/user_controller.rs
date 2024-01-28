use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde::{Deserialize, Serialize};

pub fn generate_users() -> Vec<User> {
    let mut users: Vec<User> = Vec::new();
    for i in 1..=10 {
        let user = User {
            id: i,
            username: format!("user{}", i),
        };
        users.push(user);
    }
    users
}
pub async fn root() -> &'static str {
    "Hello, World!"
}
pub async fn handler_hello() -> impl IntoResponse {
    Html("<h1 style=color:red;>Hello, World!</h1")
}
pub async fn look() -> impl IntoResponse {
    "Hello, World!".into_response()
}

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}
pub async fn get_users(Query(params): Query<Pagination>) -> (StatusCode, Json<Vec<User>>) {
    // Create a vector of 10 users
    let users = generate_users();

    let page = params.page.unwrap_or_else(|| 1);
    let limit = params.limit.unwrap_or_else(|| 10);
    println!("page is {} and limit is {}", page, limit);

    (StatusCode::OK, Json(users))
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize, Clone)]
pub struct User {
    id: u64,
    username: String,
}
