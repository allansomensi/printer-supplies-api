CREATE TABLE toners (
    id UUID PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    stock INTEGER,
    price DECIMAL(6, 2)
);
