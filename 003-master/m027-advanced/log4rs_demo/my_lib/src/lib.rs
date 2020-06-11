use log::debug;

pub struct Config;

impl Config { 
    pub fn load_global_config(){
        debug!("Configuration files loaded");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
