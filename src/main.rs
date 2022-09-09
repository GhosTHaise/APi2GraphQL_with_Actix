use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod modules;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/developper")]
async fn developper() -> impl Responder {
    HttpResponse::Ok().body("GhosT is the developper")
}

#[get("/aboutMe")]
async fn aboutme() -> web::Json<modules::aboutMe::Informations> {
    let fitiavana = modules::aboutMe::Informations::new(
        "Fitiavana".to_string(),
        "Sambatra".to_string(),
        vec![modules::aboutMe::Hobies{
            id : 1,
            name : "videogames".to_string()
        },modules::aboutMe::Hobies{
            id : 2,
            name : "coding".to_string()
        },
        modules::aboutMe::Hobies{
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

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
} 

/* #[get("temperature")]
async fn current_temperature() -> impl Responder {
    web::Json(json!({ "temperature": 42.3 })?)
} */
async fn index() -> impl Responder {
    "Hello world"
}
struct AppState{
    app_name : String
}

#[get("/state")]
async fn state(data : web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(developper)
            .service(aboutme)
            .service(
                web::scope("/app").route("/index.html",web::get().to(index))
            )
            .app_data(web::Data::new(AppState{
                app_name : String::from("Actix web")
            }))
            .service(state)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}