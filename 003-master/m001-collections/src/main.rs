

fn print_header(title: &str){
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
}

fn tuples(){
    print_header("Tuples");
    let num_and_str: (u8, &str) =(44, "Atul Sharma");
    println!("{:?}", num_and_str);
    let (num, string) = num_and_str;
    println!("Destructured - Number: {}; String: {}", num, string);
}

fn arrays() {
    print_header("Arrays");
    let numbers:[u8; 10] =[1,2,3,4,5,6,7,8,9,10];
    let floats =[0.1f64, 0.2, 0.3];

    println!("Number: {}", numbers[5]);
    println!("Float: {}", floats[2]);
}
