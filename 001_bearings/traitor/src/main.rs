mod media;
mod vehicle;

use media::Playable;
use vehicle::Car;
use vehicle::Vehicle;

struct Audio(String);
struct Video(String);

impl Playable for Audio{
    fn play(&self){
        println!("Now playing audio: {}", self.0);
    }
}
impl Playable for Video{
    fn play(&self){
        println!("Now playing video: {}", self.0);
    }
}


struct AudiA5{
    model: String,
    release_date: u16
}

impl AudiA5{
    fn new(model: &str, release_date: u16)-> Self{
        Self{model: model.to_string(), release_date}
    }
}
impl Vehicle for AudiA5{
    fn get_price(&self) -> u64{
        60_000
    }
}
impl Car for AudiA5 {
    fn model(&self)->String{
        "Audi A5 Sportsback".to_string()
    }
}

fn main() {
    let audio = Audio("Where Streets Have No Name".to_string());
    let video = Video("Casablanca".to_string());
    audio.play();
    video.play();

    let my_audi = AudiA5::new("Audi A5 Quattro", 2017);
    println!("{} is priced at £{}",
        my_audi.model(), my_audi.get_price());
}
