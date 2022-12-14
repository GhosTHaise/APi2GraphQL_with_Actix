extern crate orm_diesel;
extern crate diesel;


use self::orm_diesel::*;

fn main() {

    //use self::schema::projects::dsl::*;
    let orm : OrmDiesel = orm_diesel::OrmDiesel::new("project".to_string());
    let title = "API withb rust";
    let url = "https://GhosTHaise@github.com";
    let created_at = "23/09/2022:16:17:01";
    /* let project = create_post(&connection,title,url,created_at);
    println!("Saved project : {}, with id : {} successfully",project.title,project.id); */
    let data = orm.new_project(&title,&url,&created_at);
    orm.create_post(&data);
}