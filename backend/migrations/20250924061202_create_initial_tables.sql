-- Add migration script here
-- migrations/20250922120000_create_initial_tables.sql

-- Drop old table if exists
DROP TABLE IF EXISTS users CASCADE;

-- Create new users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    phone_number TEXT,
    address TEXT,
    secondary_email TEXT,
    office_name TEXT,
    image_url TEXT,
    user_type TEXT NOT NULL DEFAULT 'user', -- 'admin', 'manager', 'user'
    created_at TIMESTAMP DEFAULT NOW()
);


-- Box Types
CREATE TABLE box_types (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

-- Brands
CREATE TABLE brands (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    cost_per_kg NUMERIC NOT NULL
);

-- Roll Types
CREATE TABLE roll_types (
    id SERIAL PRIMARY KEY,
    roll_type TEXT NOT NULL,
    cost NUMERIC NOT NULL,
    length INT NOT NULL,
    width INT NOT NULL
);

-- Products
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    product_type TEXT NOT NULL,
    brand_id INT REFERENCES brands(id),
    roll_type_id INT REFERENCES roll_types(id),
    gsm INT,
    length INT,
    breadth INT,
    height INT,
    quantity INT,
    final_cost TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
