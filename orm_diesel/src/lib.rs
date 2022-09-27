#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde_json;
extern crate serde;

pub mod model;
pub mod schema;

use diesel::{prelude::*, insert_into};
use dotenv::dotenv;
use std::env;
use schema::projects::dsl::*;
pub struct OrmDiesel{
    pub connection : MysqlConnection
}
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in env file .");
    MysqlConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting"))
}

impl OrmDiesel{
    pub fn new(_model : String) -> OrmDiesel{
        OrmDiesel{
            connection : establish_connection()
        }
    }
      
    pub fn create_post(&self,data : &model::NewProject) -> () {
        //let json = r#"{"title":"Test Insertion","url":"sdfsdfsdf","created_at":"24 septembre"}"#;
        //let project_json = serde_json::from_str::<model::NewProject>(json).unwrap();
        /* let new_data = model::NewProject{
            title : "Bonjour ...",
            url : "asdasdas@gamilad.com",
            created_at : "24 septembre 2022"
        };
         */
        insert_into(projects).values(data).execute(&self.connection);
    }
    pub fn new_project<'a>(&self,_title : &'a str,_url : &'a str,_created_at : &'a str) -> model::NewProject<'a> {
        model::NewProject{
            title : _title,
            url : _url,
            created_at : _created_at
        }
    }

}


