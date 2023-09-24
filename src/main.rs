mod repository;
mod models;
mod api;


use actix_cors::Cors;
use actix_web::{ web::Data, middleware::Logger, get, App, HttpResponse, HttpServer, Responder, http::header};
use repository::mongodb_repo::MongoRepo;
use api::routes;


pub struct AppState {
    db: MongoRepo
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    println!("🚀 Server started successfully");


    HttpServer::new(move || {

        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(db_data.clone())//Db connection
            .wrap(Logger::default())
            .wrap(cors)
            // .service(test)
            // .service(create_user)
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
