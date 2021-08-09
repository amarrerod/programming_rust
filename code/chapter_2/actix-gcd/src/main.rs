mod computing;
mod routes;
use actix_web::{web, App, HttpServer};

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::get_index))
            .route("/gcd", web::post().to(routes::post_gcd))
    });
    println!("Serving on http://localhost:3000");
    server
        .bind("127.0.0.1:3000")
        .expect("Error binding server to address")
        .run()
        .expect("Error running server");
}
