use actix_web::{web, App, HttpServer};

// fn index() -> &'static str {
//     "Hello world!"
// }

fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            //.wrap(middleware::Logger::default())
            .service(web::resource("/").to(|| "Hello world"))
    })
    .bind("0.0.0.0:8080")?
    .run()
}
