-- Add migration script here
CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    price DOUBLE PRECISION NOT NULL,
    quantity DOUBLE PRECISION NOT NULL
);