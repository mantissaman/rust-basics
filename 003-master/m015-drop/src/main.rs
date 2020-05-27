struct Character {
    name: String,
}

impl Drop for Character {
    fn drop(&mut self) {
        println!("{} dropped", self.name)
    }
}

fn main() {
    let atul = Character {name: "Atul".into() };
    let Neil = Character {name: "Neil".into() };
}
