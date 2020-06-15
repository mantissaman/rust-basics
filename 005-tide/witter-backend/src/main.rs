extern crate pretty_env_logger;
#[macro_use] extern crate log;

use dotenv;
use sqlx::{Pool, PgPool, query};
use tide::{Server, Request, Response, Body};
use tide::http::StatusCode;
use serde_json::json;
use serde_json::{Map, Value};


#[async_std::main]
async fn main() ->Result<(),Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let app = server().await?;
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn server() -> Result<Server<State>,Error>  {
    let db_url = std::env::var("DATABASE_URL")?;
    let db_pool:PgPool = Pool::new(&db_url).await?;
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
    Ok(app)
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
    VarError(#[from] std::env::VarError),
}


#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use http_service::{HttpService,Response};
    use futures::{executor::block_on, prelude::*};
    use http_types::{Method, Request, Url};

    #[async_std::test]
    async fn a_test(){

    }

    #[async_std::test]
    async fn nested(){
        let server =  server().await.unwrap();
        let mut server = make_server(server).unwrap();

        let req = Request::new(
            Method::Get,
            Url::parse("http://example.com/").unwrap(),
        );
        let res = server.simulate(req).unwrap();
        assert_eq!(res.status(), "200");
    }

    pub fn make_server<T: HttpService>(service: T) -> Result<TestBackend<T>, <T::ConnectionFuture as TryFuture>::Error>{
        TestBackend::wrap(service)
    }

    #[derive(Debug)]
    pub struct TestBackend<T: HttpService>{
        service: T,
        connection: T::Connection,
    }

    impl<T: HttpService> TestBackend<T>{
        fn wrap(service: T) -> Result<Self, <T::ConnectionFuture as TryFuture>::Error>{
            let connection = block_on(service.connect().into_future())?;
            Ok(Self {
                service,
                connection,
            })
        }

        pub fn simulate(&mut self, req:Request) -> Result<Response, <T::ResponseFuture as TryFuture>::Error>{
            block_on(
                self.service
                .respond(self.connection.clone(), req)
                .into_future()
            )
        }
    }

}