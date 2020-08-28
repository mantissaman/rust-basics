
#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    fav_color: Color
}

//Give Person some abilities
impl Person {
    pub fn print(self) -> String {
        format!("name = {}, age = {} has {} children",
        self.name, self.age, self.children)
    }
}

#[derive(Debug)]
pub enum Color {
    Red(String),
    Green,
    Blue
}

fn main() {
    let p = Person {
        name: "Atul".to_string(),
        age: 42,
        children: 2,
        fav_color: Color::Blue
    };
    let c= Color::Red("pale".to_string());

    match c {
        Color::Red(s) => println!("It's {} Red", s),
        Color::Green => println!("It's Green"),
        Color::Blue => println!("It's Blue"),
    };

    println!("Hello, people, from {:?}", p);
    println!("Hello, people, from {}", p.print());

}
