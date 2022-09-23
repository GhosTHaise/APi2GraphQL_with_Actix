use diesel::{Insertable, Queryable};
#[derive(Queryable)]
pub struct Links {
    pub id: i32,
    pub link: String,
    pub title: String,
    pub date_created: Option<String>,
}

#[derive(Insertable)]
#[table_name = links]
pub struct NewLinks<'a> {
    pub link: &'a str,
    pub title: &'a str,
    pub age: &'a i32,
}
