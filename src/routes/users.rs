use rocket::serde::json::{Value, Json};
use initialref::models::models::{UserInputUser,UserInputUpdateUser};

// import services module
use crate::services;


#[post("/add-user", format = "json", data = "<user_info>")]
pub fn create_user(user_info: Json<UserInputUser>) -> Value {
    services::users::create_user(&user_info)
}

#[get("/get-user")]
pub fn get_user()-> Value {
    services::users::get_users()
}

#[put("/update-user", format = "json", data = "<user_info>")]
pub fn update_user(user_info: Json<UserInputUpdateUser>) -> Value {
    services::users::update_user(&user_info)
}