
extern crate slab;
extern crate futures;
extern crate hyper;
extern crate regex;
#[macro_use]
extern crate lazy_static;


use futures::future;
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::{Body, Error, Request, Response, Server};
use hyper::{Method, StatusCode};
use slab::Slab;
use std::sync::{Mutex,Arc};
use std::fmt;
use regex::Regex;

/*
Slab is and allocator that can store and remove any value identified
by an ordered number. It can also reuse the slots of removed Items.
Its like Vc but do not resize on removals.
*/



type UserId = u64;
struct UserData;
impl fmt::Display for UserData{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        f.write_str("{}")
    }
}

type UserDb = Arc<Mutex<Slab<UserData>>>;

type BoxFut = Box<Future<Item = Response<Body>, Error = Error> + Send>;

lazy_static! {
    static ref INDEX_PATH: Regex = Regex::new("^/(index\\.html?)?$").unwrap();
    static ref USER_PATH: Regex = Regex::new("^/user/((?P<user_id>\\d+?)/?)?$").unwrap();
    static ref USERS_PATH: Regex = Regex::new("^/users/?$").unwrap();
}


const INDEX: &'static str = r#"
 <!doctype html>
 <html>
     <head>
         <title>Rust Microservice</title>
     </head>
     <body>
         <h3>Rust Microservice</h3>
     </body>
 </html>
 "#;
 fn handle_users(req: &Request<Body>, res: &mut Response<Body> , user_db: &UserDb){
    let users = user_db.lock().unwrap();
    match req.method(){
        &Method::GET => {
            let list = users.iter()
            .map(|(id, _)| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
            *res.body_mut() = list.into()
        },
        _ => *res.status_mut() = StatusCode::METHOD_NOT_ALLOWED
    }
 }
fn handle_user(req: &Request<Body>, res: &mut Response<Body>, user_id:Option<usize> , user_db: &UserDb){
    let mut users = user_db.lock().unwrap();

    match (req.method(), user_id){
        (&Method::GET, Some(id)) =>{
            if let Some(data) = users.get(id) {
                *res.body_mut() = data.to_string().into()
            } else {
                *res.status_mut() = StatusCode::NOT_FOUND
            }
        },
        (&Method::PUT, Some(id)) =>{
            if let Some(user) = users.get_mut(id) {
                *user = UserData;
                *res.status_mut() = StatusCode::OK
            } else {
                *res.status_mut() = StatusCode::NOT_FOUND
            }
        },
        (&Method::POST, None) => {
            let id = users.insert(UserData);
            *res.body_mut() = id.to_string().into()
        },
        (&Method::DELETE, Some(id)) =>{
            if users.contains(id) {
                users.remove(id);
                *res.status_mut() = StatusCode::OK
            } else {
                *res.status_mut() = StatusCode::NOT_FOUND
            }
        },
        (&Method::POST, Some(_)) => *res.status_mut() = StatusCode::BAD_REQUEST,
        _ => *res.status_mut() = StatusCode::METHOD_NOT_ALLOWED
    }

}

fn handle_root(_: &Request<Body>, res: &mut Response<Body>){
    *res.body_mut() = INDEX.into();
}
fn response_with_code(res: &mut Response<Body>, status_code: StatusCode){
    *res.status_mut() = status_code;
}

///Router
fn microservice_handler(req: Request<Body>, user_db: &UserDb) -> BoxFut {
    let mut response = Response::new(Body::empty());
    let path = req.uri().path();
    let method = req.method();

    println!("Path--> {}, Method--> {}",path,method);
    if INDEX_PATH.is_match(path){
        println!("INDEX");
        if method == &Method::GET {
            handle_root(&req, &mut response)
        }
        else{
            response_with_code(&mut response, StatusCode::METHOD_NOT_ALLOWED)
        }
    } else if USERS_PATH.is_match(path){
        println!("USERS");
        if method == &Method::GET {
            handle_users(&req, &mut response, &user_db) 
        }
        else{
            response_with_code(&mut response, StatusCode::METHOD_NOT_ALLOWED)
        }
    } else if let Some(cap) = USER_PATH.captures(path){
        println!("USER");
        let user_id = cap.name("user_id").and_then( |m| {
            m.as_str()
            .parse::<UserId>()
            .ok()
            .map(|x| x as usize)
        });   
        handle_user(&req, &mut response, user_id, &user_db)  
    }
    else{
        println!("HO!");
        response_with_code(&mut response, StatusCode::NOT_FOUND)
    }

    Box::new(future::ok(response))
}

fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    let user_db:UserDb = Arc::new(Mutex::new(Slab::new()));
    let server = Server::bind(&addr)
        .serve(move || {
            let user_db = user_db.clone();
            service_fn(move |req| microservice_handler(req, &user_db))
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
