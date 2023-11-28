use diesel:: prelude::*;
use initialref::{
    models::models::{NewUser,UserInputUser,User,UserInputUpdateUser,UserInputEmail},
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



pub fn update_user(user_details: &UserInputUpdateUser) -> Value {
    use initialref::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let existing_user = users
    .filter(email.eq(user_details.email.clone().unwrap()))
    .limit(1)
    .load::<User>(connection)
    .expect("Error fetching user");

    let hashed: String;

    let updated_user_body: NewUser = NewUser {
        id: &existing_user[0].id,
        name: &user_details.name.clone().unwrap_or(existing_user[0].name.clone()),
        phone: &user_details.phone.clone().unwrap_or(existing_user[0].phone.clone()),
        email: &user_details.email.clone().unwrap_or(existing_user[0].email.clone()),
        password: match &user_details.password {
            Some(new_password) => {
                hashed = hash(new_password, DEFAULT_COST).unwrap();
                &hashed
            },
            None => &existing_user[0].password,
        },
    };
    
    let updated_user: User = diesel::update(users.filter(email.eq(user_details.email.clone().unwrap())))
    .set(&updated_user_body)
    .get_result::<User>(connection)
    .expect("Error updating user");

    json!(updated_user)
}



pub fn delete_user(user_details: &UserInputEmail) -> Value {
     use initialref::schema::users::dsl::*;
    //  println!("Received email for deletion: {}", &email.email);

    let connection = &mut establish_connection();
    

    // Check if the user exists before attempting to delete
    let existing_user = users
        .filter(email.eq(&user_details.email))
        .limit(1)
        .load::<User>(connection)
        .expect("Error fetching user");

    if existing_user.is_empty() {
        return json!({
            "error": "User not found",
        });
    }

    // Delete the user
    let deleted_user: User = diesel::delete(users.filter(email.eq(&user_details.email)))
        .get_result(connection)
        .expect("Error deleting user");

    json!({
        "message": "User deleted successfully",
        "user":deleted_user,
    })
}
