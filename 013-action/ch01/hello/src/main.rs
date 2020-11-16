fn greet() {
    println!("Hello, world!");
    let german = "Grüß Gott!";
    let japanese = "ハロー・ワールド";
    let languages =[german, japanese];

    for language in languages.iter() {
        println!("{}", language);
        println!("{}", &language);
    }
}

fn main() {
    greet();
}
