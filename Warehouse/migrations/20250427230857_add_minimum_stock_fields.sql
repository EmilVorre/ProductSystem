-- Add migration script here

CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    price DOUBLE PRECISION NOT NULL,
    quantity DOUBLE PRECISION NOT NULL,
    minimum_stock FLOAT DEFAULT 0,
    pack_size INT DEFAULT 1,
    distributor TEXT DEFAULT ''
);
