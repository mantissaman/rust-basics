pub trait Vehicle {
    fn get_price(&self) ->u64;
}
pub trait Car: Vehicle {
    fn model(&self) -> String;
}