-- Your SQL goes here
CREATE TABLE configs (
	filename VARCHAR NOT NULL PRIMARY KEY,
	shorthand VARCHAR NOT NULL,
	content TEXT NOT NULL
);