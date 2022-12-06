-- Your SQL goes here
CREATE TABLE secrets (
	project VARCHAR NOT NULL,
	path VARCHAR NOT NULL,
	content TEXT NOT NULL,
	PRIMARY KEY (project, path)
);