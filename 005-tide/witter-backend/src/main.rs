extern crate pretty_env_logger;
#[macro_use] extern crate log;

use dotenv;
use sqlx::{Pool, PgPool, query};
use tide::{Server, Request, Response, Body};
use tide::http::StatusCode;
use serde_json::json;
use serde_json::{Map, Value};


#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let app = server().await;
    app.listen("127.0.0.1:8080").await.unwrap();

}

async fn server() -> Server<State>  {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool:PgPool = Pool::new(&db_url).await.unwrap();
    let mut app:Server<State> = Server::with_state(State{db_pool});

    app.at("/").get(|req: Request<State>| async move { 
        let db_pool: &PgPool = &req.state().db_pool;
        let rows = query!("select 1 as one").fetch_one(db_pool).await?;
        dbg!(&rows);
        //Ok(format!("Hello, world- {}!", rows.one.unwrap())) 
        
        let mut map = Map::new();
        map.insert("one".to_owned(), json!(&rows.one));
        map.insert("two".to_owned(), json!(true));
        let json = Value::Object(map);

        //Ok(json)
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(Body::from_json(&json)?);
        Ok(res)
    });
    app
}

#[derive(Debug)]
struct State{
    db_pool: PgPool,
}


#[derive(thiserror::Error, Debug)]
enum Error{
    #[error(transparent)]
    DbError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError)
}


#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use http_service::Response;
    use http_service_mock::{make_server, TestBackend};
    use serde::de::DeserializeOwned;
    use serde_json::json;
    use tide::server::Service;



    pub struct TestApp {
        pub server: TestBackend<Service<State>>
    }

    impl TestApp {
        pub fn new() -> Self {
            let app = server();
            let server = make_server(app.into_http_service()).unwrap();
            Self {
                server
            }
        }
    }

    #[async_std::test]
    async fn nested(){
        let server =  server().await;
        let mut server = make_server(server);       
    }

    
}