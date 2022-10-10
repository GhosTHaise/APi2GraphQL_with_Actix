
use diesel;
use serde::{Serialize,Deserialize};

use crate::*;


use crate::schema::projects::dsl::*;
use diesel::prelude::*;

#[derive(Serialize,Deserialize)]
pub struct Project;

use crate::schema::projects::dsl::*;

impl Project{
    pub fn select() -> Vec<crate::model::Project> {
    let connection =  establish_connection();
    let result = projects
        .load::<crate::model::Project>(&connection)
        .expect("Error loading students");

        println!(
            "Project : {},{},{} <>",
            result[0].title,
            result[0].url,
            result[0].created_at
        );  
        result
    }
}