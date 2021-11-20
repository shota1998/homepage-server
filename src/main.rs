#[macro_use] extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer};
use actix_service::Service;

mod schema;
mod database;
mod processes;
mod models;
mod state;
mod to_do;
mod json_serialization;
mod views;
mod auth;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                    // srv => routing
                    // req => service request
                if *&req.path().contains("/item/") {
                    match auth::process_token(&req) {
                        Ok(_token) => println!("the token is passable"),
                        Err(message) => println!("token error: {}", message)
                    }
                }
                let fut = srv.call(req);
                async {
                    let result = fut.await?;
                    Ok(result)
                }
            }).configure(views::views_factory);
        return app
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
