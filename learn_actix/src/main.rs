use actix_cors::Cors;
use actix_web::{get, post, web,http, App, HttpResponse, HttpServer, Responder, middleware::Logger};

mod modules;
mod api;
mod repository;

use api::task::{
    get_task,
    get_project,
    insert_project
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        println!("Service starts at : http://127.0.0.1:{}",8080);
        let cors = Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET","POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION,http::header::ACCEPT,http::header::ORIGIN,http::header::ACCESS_CONTROL_ALLOW_ORIGIN])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);

        //app
        App::new()
            .wrap(logger)
            //middleware cors
            .wrap(cors)
            //learn actix tutorial
            .service(get_task)
            .service(insert_project)
            //end
            .service(get_project)
            .service(modules::old_task::hello)
            .service(modules::old_task::echo)
            .service(modules::old_task::developper)
            .service(modules::old_task::aboutme)
            .service(
                web::scope("/app").route("/index.html",web::get().to(modules::old_task::index))
            )
            .app_data(web::Data::new(modules::old_task::AppState{
                app_name : String::from("Actix web")
            }))
            .service(modules::old_task::state)
            .route("/hey", web::get().to(modules::old_task::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}