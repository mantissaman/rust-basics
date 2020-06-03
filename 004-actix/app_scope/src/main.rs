use actix_web::{web, App, Responder, HttpServer};
use std::sync::Mutex;

async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("{} called {} times.", app_name, counter)
}

struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    let app_state= web::Data::new(AppState{
        app_name: String::from("Actix-Web"),
        counter: Mutex::new(0)
    });
    HttpServer::new(move || {
        // App::new().service(
        //     web::scope("/")
        //     .data(AppState {
        //         app_name: String::from("Actix-Web")
        //     })
        //     .route("/index.html", web::get().to(index)),
        // )
        App::new()
            .app_data(app_state.clone())
            .service(
                web::scope("/")
                .route("/index.html", web::get().to(index))
            )
            
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
