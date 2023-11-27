use diesel:: prelude::*;
use rocket::serde::{Serialize, Deserialize};

use crate::schema::users;

/*
* User models begin from here
*/

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, AsChangeset)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub phone: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}


#[derive(Deserialize)]
pub struct UserInputUser {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInputUpdateUser {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
