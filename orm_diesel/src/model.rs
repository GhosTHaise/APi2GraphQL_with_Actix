use super::schema::projects;
use serde::{Deserialize,Serialize};
use diesel::{Insertable, Queryable};

#[derive(Serialize,Deserialize,Debug)]
#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub created_at: String
}

#[derive(Deserialize,Serialize,Insertable)]
#[table_name = "projects"]
pub struct NewProject<'a> {
    #[serde(borrow)]
    pub title: &'a str,
    #[serde(borrow)]
    pub url: &'a str,
    #[serde(borrow)]
    pub created_at: &'a str,
}
