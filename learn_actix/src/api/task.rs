use actix_web::{
    get,
    post,
    web::Path,
    web::Json,
    web::Form,
    HttpResponse,
};
use orm_diesel;
use serde::{Serialize,Deserialize};
use std::{thread};
use std::time::Duration;
#[derive(Serialize,Deserialize)]
pub struct TaskIdentifier{
    task_global_id : String
}

#[derive(Serialize,Deserialize)]
pub struct FormulaireProject{
    title : String,
    url : String,
    created_at : String
}

#[get("/project/list")]
pub async fn get_project() -> Json<Vec<orm_diesel::model::Project>>{
        let data  = orm_diesel::query::Project::Project::select();
        Json(data)
}
#[get("/task/{task_global_id}")]
pub async fn get_task(task_indentifier : Path<TaskIdentifier>) -> Json<TaskIdentifier> {
    thread::sleep(Duration::from_secs(5));
    Json(task_indentifier.into_inner())
}
#[post("/project/new")]
pub async fn insert_project<'a>(form : Form<FormulaireProject>) -> HttpResponse {

    orm_diesel::query::Project::Project::insert(&*form.title, &*form.url, &*form.created_at);
    HttpResponse::Ok().body(format!("data : {}",form.url))
}

#[get("/project/remove/{id_project}")]
pub async fn remove_project(project_info : Path<String>) -> HttpResponse {
    let id = project_info.into_inner();
    orm_diesel::query::Project::Project::detele(id);
    HttpResponse::Ok().body("Remove project successfully")
}

#[post("/project/update/{id_project}")]
pub async fn update_project(form : Form<FormulaireProject>,params : Path<String>) -> HttpResponse {
    let id  = params.into_inner();
    let update_id = id.parse().unwrap();
    let _uptade_project = orm_diesel::model::Project{
        id : update_id,
        title : String::from(&*form.title),
        url : String::from(&*form.url),
        created_at : String::from(&*form.created_at)
    };
    HttpResponse::Ok().body(format!("Update project(id : {})successfully",id))
}