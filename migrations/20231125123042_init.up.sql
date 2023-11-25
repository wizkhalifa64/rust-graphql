-- Add up migration script here
CREATE TABLE "users" (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100),
    age SMALLINT,
    email VARCHAR(255) NOT NULL UNIQUE,
    photo VARCHAR NOT NULL DEFAULT 'default.png',
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    password VARCHAR(100) NOT NULL,
    role SMALLINT NOT NULL DEFAULT 2,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    FOREIGN KEY(role) REFERENCES roles(role_id)
);

CREATE INDEX users_email_idx ON users (email);