
use diesel::{self, update, connection};
use serde::{Serialize,Deserialize};

use crate::*;


use crate::model::NewProject;
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
    
    pub fn insert(_title : &str,_url : &str,_created_at : &str) -> () {
        let orm = OrmDiesel::new("Project".to_string());
        let data = orm.new_project(
            _title,
            _url,
            _created_at
        );
        orm.create_post(&data);
    }   
      
    pub fn detele(id_project : String) -> () {
        let connection = establish_connection();
        let parsed_params : i32 = id_project.parse().unwrap();
        delete(projects.filter(id.eq_all(parsed_params))).execute(&connection);
    }

    pub fn update(change_project : crate::model::Project) -> () {
        let connection = establish_connection();
        let result = update(projects.filter(id.eq(change_project.id)))
            .set(
                (title.eq_all(change_project.title),
                        url.eq_all(change_project.url),
                        created_at.eq_all(change_project.created_at))
            ).execute(&connection);
    }
}