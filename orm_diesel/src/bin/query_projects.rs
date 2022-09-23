extern crate orm_diesel;
extern crate diesel;

use self::model::*;
use orm_diesel::*;
use diesel::prelude::*;

fn main() {
    use self::schema::projects::dsl::*;

    let connection =  establish_connection();
    let result = projects
        .filter(id.eq(1))
        .load::<Project>(&connection)
        .expect("Error loading students");

    println!(
        "Project : {},{},{} <>",
        result[0].title,
        result[0].url,
        result[0].created_at
    ); 
}