-- Add migration script here

CREATE TABLE IF NOT EXISTS countries (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    code CHAR(2) NOT NULL UNIQUE,
    capital VARCHAR(100),
    region VARCHAR(100),
    subregion VARCHAR(100),
    population BIGINT,
    area DOUBLE, -- km2
    currency VARCHAR(50),
    language VARCHAR(50),
    flag_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE IF NOT EXISTS cities (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    country_code CHAR(2),
    region VARCHAR(100),
    population BIGINT,
    latitude DOUBLE,
    longitude DOUBLE,
    timezone VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (country_code) REFERENCES countries(code)
);

