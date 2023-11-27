use diesel:: prelude::*;
use initialref::{
    models::models::{NewUser,UserInputUser,User},
    *,
};

use rocket::serde::json::{json, Value};

extern crate bcrypt;
use bcrypt::{hash, DEFAULT_COST};

pub fn create_user(user_details: &UserInputUser) -> Value {
    use initialref::schema::users;

    let connection = &mut establish_connection();



    let user_id = uuid::Uuid::new_v4().to_string();
    let hashed = hash(&user_details.password, DEFAULT_COST).unwrap();

    let new_user: NewUser = NewUser {
        id: &user_id,
        name: &user_details.name,
        phone: &user_details.phone,
        email: &user_details.email,
        password: &hashed,
    };

    let created_user: User = diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(connection)
        .expect("Error saving new user");

    json!(created_user)
}   



pub fn get_users() -> Value {
    use initialref::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let results: Vec<User> = users.load::<User>(connection).expect("Error loading posts");

    json!(results)
}