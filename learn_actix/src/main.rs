use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};

mod modules;
mod api;
mod repository;

use api::task::{
    get_task
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        println!("Service starts at : http://127.0.0.1:{}",8080);
        App::new()
            .wrap(logger)
            //learn actix tutorial
            .service(get_task)
            //end
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