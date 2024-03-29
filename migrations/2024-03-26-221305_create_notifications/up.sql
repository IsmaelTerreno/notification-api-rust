-- Your SQL goes here
CREATE TABLE notification
(
    id    UUID PRIMARY KEY,
    topic VARCHAR NOT NULL,
    body  TEXT    NOT NULL,
    read  BOOLEAN NOT NULL DEFAULT FALSE
)

