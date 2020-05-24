trait Vehicle {
    fn get_price(&self) ->u64;
}

trait Car: Vehicle {
    fn model(&self) -> String;
}

struct Audi{
    model: String,
    release_date: u16
}

impl Audi {
    fn new(model: &str, release_date: u16) -> Self{
        Self { model: model.to_string(), release_date}
    }
}

impl Car for Audi{
    fn model(&self) -> String {
        self.model.to_string()
    }
}

impl Vehicle for Audi{
    fn get_price(&self) ->u64 {
        50_000
    }
}



fn main(){
    let my_audi = Audi::new("A5", 2017);
    println!("{} is priced at £{}", my_audi.model, my_audi.get_price());
}