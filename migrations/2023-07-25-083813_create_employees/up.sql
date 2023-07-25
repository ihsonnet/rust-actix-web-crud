
CREATE TABLE IF NOT EXISTS "employees"
(
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    department VARCHAR NOT NULL,
    salary INT NOT NULL,
    age INT NOT NULL


)
