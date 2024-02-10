use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = super::schema::configs)]
pub struct Config {
    pub filename: String,
    pub shorthand: String,
    pub content: String,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = super::schema::secrets)]
pub struct Secret {
    pub project: String,
    pub path: String,
    pub content: String,
}
