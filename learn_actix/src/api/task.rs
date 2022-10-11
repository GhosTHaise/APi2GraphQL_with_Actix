use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType,StatusCode}
};
use orm_diesel;
use serde::{Serialize,Deserialize};
use derive_more::{Display};
use std::thread;
use std::time::Duration;
#[derive(Serialize,Deserialize)]
pub struct TaskIdentifier{
    task_global_id : String
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
