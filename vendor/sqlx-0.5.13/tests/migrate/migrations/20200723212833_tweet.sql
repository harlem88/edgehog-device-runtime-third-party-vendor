CREATE TABLE tweet (
    id BIGINT NOT NULL PRIMARY KEY,
    text TEXT NOT NULL,
    is_sent BOOLEAN NOT NULL DEFAULT TRUE,
    owner_id BIGINT
);
