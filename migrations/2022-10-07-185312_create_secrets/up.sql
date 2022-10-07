-- Your SQL goes here
CREATE TABLE secrets (
	project VARCHAR NOT NULL PRIMARY KEY,
	path VARCHAR NOT NULL,
	content TEXT NOT NULL
);