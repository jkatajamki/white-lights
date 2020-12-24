-- Your SQL goes here

CREATE TABLE IF NOT EXISTS wl_users (
  user_id BIGSERIAL NOT NULL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL,
  email TEXT NOT NULL UNIQUE,
  user_secret TEXT NOT NULL,
  updated_at TIMESTAMP DEFAULT NULL
);
