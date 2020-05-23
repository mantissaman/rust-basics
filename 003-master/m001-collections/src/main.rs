use std::collections::HashMap;

fn print_header(title: &str) {
    println!("");
    //This centers the title within a line of n - characters.
    //Use {:-<1$} for left alignment, and {:->1$} for right alignment.
    let formatted_title = format!("{:-^1$}", title, 20);
    println!("{}", formatted_title);
    //println!("{}", (0..20).map(|_| "_").collect::<String>());
}

fn main() {
    arrays();
    tuples();
    vectors();
    hashmaps();
    slices();
    count_words();
}

#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> Self {
        WordCounter(HashMap::new())
    }

    fn increment(&mut self, word: &str){
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count +=1;
    }

    fn display(&self) {
        for (key, value) in self.0.iter().filter(|&(_, v)| *v > 1 ) {
            println!("{} : {}", key, value);
        }
    }
}

//use std:: env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::BufRead;

fn count_words() {
    print_header("Count Words");
    //let args: Vec<String> = env::args().collect();
    //let filename = &args[1];
    let filename ="test.txt";
    println!("Processing file: {}", filename);

    let file = File::open(filename).expect("could not open file");
    let reader = BufReader::new(file);

    let mut word_counter =WordCounter::new();

    for line in reader.lines(){
        let line = line.expect("could not read line");
        let words  = line.split(" ");
        for word in words {
            if word =="" {
                continue;
            }
            else{
                word_counter.increment(word);
            }
        }
    }
    word_counter.display();
}

fn slices() {
    print_header("Slices");
    let mut numbers: [u8; 4] = [1, 2, 3, 40];
    let all: &[u8] = &numbers[..];
    println!("All of them: {:?}", all);

    let first_two: &mut [u8] = &mut numbers[0..2];
    first_two[0] = 100;
    first_two[1] = 99;
    println!("Modified: {:?}", numbers);
}

fn hashmaps() {
    print_header("HashMaps");
    let mut fruits = HashMap::new();
    fruits.insert("apple", 3);
    fruits.insert("mango", 6);
    fruits.insert("orange", 2);
    fruits.insert("avocado", 7);

    for (k, v) in &fruits {
        println!("I got {} {}s", v, k);
    }
    fruits.remove("orange");

    let old_avocado = fruits["avocado"];
    fruits.insert("avocado", old_avocado + 5);

    println!("Now have {} avocados", fruits["avocado"]);
}

fn vectors() {
    print_header("Vectors");
    let mut numbers_vec: Vec<u8> = Vec::new();
    numbers_vec.push(1);
    numbers_vec.push(2);

    let mut vec_with_macro = vec![1];
    vec_with_macro.push(2);

    let message = if vec_with_macro == numbers_vec {
        "They are equal"
    } else {
        "Nah! They are different"
    };

    println!("{} {:?} {:?}", message, numbers_vec, vec_with_macro);
}

fn tuples() {
    print_header("Tuples");
    let num_and_str: (u8, &str) = (44, "Atul Sharma");
    println!("{:?}", num_and_str);
    let (num, string) = num_and_str;
    println!("Destructured - Number: {}; String: {}", num, string);
}

fn arrays() {
    print_header("Arrays");
    let numbers: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let floats = [0.1f64, 0.2, 0.3];

    println!("Number: {}", numbers[5]);
    println!("Float: {}", floats[2]);
}
