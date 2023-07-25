CREATE TABLE IF NOT EXISTS "accounts"
(
    "id" SERIAL NOT NULL PRIMARY KEY,
    "href" VARCHAR(255) NOT NULL,
    "description" VARCHAR(255) NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "base_type" VARCHAR(255) NOT NULL,
    "schema_location" VARCHAR(255) NOT NULL,
    "a_type" VARCHAR(255) NOT NULL,
    "referred_type" VARCHAR(255) NOT NULL
)