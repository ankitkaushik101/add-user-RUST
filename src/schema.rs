// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        phone -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}
