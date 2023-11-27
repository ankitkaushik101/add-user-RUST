-- Your SQL goes here
CREATE TABLE users (
  id VARCHAR PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  phone VARCHAR NOT NULL,
  email VARCHAR NOT NULL UNIQUE CONSTRAINT proper_email CHECK (email ~* '^[A-Za-z0-9._+%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'),
  password VARCHAR NOT NULL
)