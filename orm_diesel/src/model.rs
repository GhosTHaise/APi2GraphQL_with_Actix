use super::schema::projects;

use diesel::{Insertable, Queryable};
#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub created_at: String
}

#[derive(Insertable)]
#[table_name = "projects"]
pub struct NewProject<'a> {
    pub title: &'a str,
    pub url: &'a str,
    pub created_at: &'a str,
}
