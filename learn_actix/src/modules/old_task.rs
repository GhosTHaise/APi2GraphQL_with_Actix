
use actix_web::{
    get,
    post,
    Responder,
    web::{Json, self},
    HttpResponse,
    http::{StatusCode}
};



use super::aboutMe;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../app/index.html") )
}
#[get("/developper")]
async fn developper() -> impl Responder {
    HttpResponse::Ok().body("GhosT is the developper")
}

#[get("/aboutMe")]
async fn aboutme() -> web::Json<aboutMe::Informations> {
    let fitiavana = aboutMe::Informations::new(
        "Fitiavana".to_string(),
        "Sambatra".to_string(),
        vec![aboutMe::Hobies{
            id : 1,
            name : "videogames".to_string()
        },aboutMe::Hobies{
            id : 2,
            name : "coding".to_string()
        },
        aboutMe::Hobies{
            id : 3,
            name : "playing chess".to_string()
        }
        ],
        19
    );
   web::Json(fitiavana)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
} 

/* #[get("temperature")]
async fn current_temperature() -> impl Responder {
    web::Json(json!({ "temperature": 42.3 })?)
} */
pub async fn index() -> impl Responder {
    "Hello world"
}
pub struct AppState{
    pub app_name : String
}

#[get("/state")]
async fn state(data : web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}
