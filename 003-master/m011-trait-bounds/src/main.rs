struct Game;
struct Enemy;
struct Hero;


trait Loadable {
    fn init(&self);
}

impl Loadable for Enemy {
    fn init(&self){
        println!("Enemy is loaded");
    }
}
impl Loadable for Hero {
    fn init(&self){
        println!("Hero is loaded");
    }
}

impl Game {
    fn load<T: Loadable>(&self, entity:T){
        entity.init();
    }
}
use std::ops::Add;

fn add_thing<T: Add> (a: T, b: T) {
    let _ = a + b;
}

fn main() {
   let game = Game;
   game.load(Enemy);
   game.load(Hero);

   add_thing (2, 3);
}
