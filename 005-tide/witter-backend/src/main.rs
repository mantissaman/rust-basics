extern crate pretty_env_logger;
#[macro_use] extern crate log;

use dotenv;
use sqlx::{Pool, PgPool, query};
use tide::{Server, Request, Response, Body};
use tide::http::StatusCode;
use serde_json::json;

#[async_std::main]
async fn main() ->Result<(),Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_url = std::env::var("DATABASE_URL")?;
    let db_pool:PgPool = Pool::new(&db_url).await?;


    let mut app:Server<State> = Server::with_state(State{db_pool});

    app.at("/").get(|req: Request<State>| async move { 
        let db_pool: &PgPool = &req.state().db_pool;
        // let rows = query!("select 1 as one where 1=2").fetch_one(db_pool).await?;
        // dbg!(&rows);
        //Ok(format!("Hello, world- {}!", rows.one.unwrap())) 


        let json = json!([1,2,3]);
        //Ok(json)
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(Body::from_json(&json)?);
        Ok(res)
    });
    app.listen("127.0.0.1:8080").await?;

    Ok(())
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
