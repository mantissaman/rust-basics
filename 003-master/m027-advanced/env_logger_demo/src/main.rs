use log::debug;
use user_auth::User;

fn main() {
   // std::env::set_var("RUST_LOG", "user_auth=info,env_logger_demo=info cargo run");

    env_logger::init();
    debug!("env logger demo started");
    let user= User::new("bob", "secret");
    user.sign_in("notsecret");
    user.sign_in("secret");
}

//RUST_LOG=user_auth=info,env_logger_demo=info cargo run
