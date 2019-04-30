

struct Game;
struct Enemy;
struct Hero;

trait Loadable{
    fn init(&self);
}

impl Loadable for Enemy{
    fn init(&self){
        println!("Enemy Loaded");
    }
}
impl Loadable for Hero{
    fn init(&self){
        println!("Hero Loaded");
    }
}
impl Game {
    // Here it is Trait bound
    fn load<T: Loadable>(&self, entity: T){
        entity.init();
    }
}

fn main() {
    let game = Game;
    game.load(Enemy);
    game.load(Hero);
}
