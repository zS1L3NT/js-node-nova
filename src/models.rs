use diesel::prelude::*;

#[derive(Queryable)]
pub struct Config {
    pub filename: String,
    pub shorthand: String,
    pub content: String,
}

#[derive(AsChangeset, Insertable, Queryable)]
#[diesel(table_name = super::schema::secrets)]
pub struct Secret {
    pub project: String,
    pub path: String,
    pub content: String,
}
