
use diesel;
use serde::{Serialize,Deserialize};

use crate::*;


use crate::schema::projects::dsl::*;
use diesel::prelude::*;

#[derive(Serialize,Deserialize)]
pub struct Project;

use crate::schema::projects::dsl::*;

impl Project{
    pub fn select() -> crate::model::Project {
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
        crate::model::Project{
             id: result[0].id,
             title: result[0].title.to_string(),
             url: result[0].url.to_string(),
             created_at: result[0].created_at.to_string()
        }
    }
}