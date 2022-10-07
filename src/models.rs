use diesel::prelude::*;

#[derive(Queryable)]
pub struct Config {
	pub filename: String,
	pub shorthand: String,
	pub content: String,
}

#[derive(Queryable)]
pub struct Secret {
	pub project: String,
	pub path: String,
	pub content: String,
}