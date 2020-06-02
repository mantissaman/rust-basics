
#[derive(Debug)]
enum Food {
    Pizza,
    Salad
}

#[derive(Debug)]
enum PaymentMode {
    Bitcoin,
    Credit
}

#[derive(Debug)]
struct Order {
    count: u8,
    item: Food,
    payment: PaymentMode
}


#[derive(Debug)]
enum MayBeContainer{
    Item(u64),
    Empty
}

fn destructure_enum(){
    let maybe_item = MayBeContainer::Item(0u64);
    let has_item = if let MayBeContainer::Item(0) = maybe_item {
        true
    } else{
        false
    };
    println!("{}", has_item);
}

#[derive(Debug)]
struct Container{
    items_count: u32
}

fn incr(Container{mut items_count}: &mut Container){
    items_count +=1;
}

fn calc(Container{ items_count}: &Container) ->u32 {
    let rate = 67;
    rate * items_count
}

fn destructure_fn(){
    let mut container = Container {items_count: 10};

    incr(&mut container);
    let total = calc(&container);

    println!("Total Cost: {}", total);
}


fn main() {
    let mut food_order = Order{ count: 2,
        item: Food::Pizza,
        payment: PaymentMode::Credit
    };
    {
        let Order {count, item, ..} = &food_order;
        // above can be written as 
        // let Order {ref count, ref item, ..} = food_order;
        println!("{},{:?},{:?}", count, item, food_order);
    }
    let Order {count, item, ..} = &mut food_order;
     *count=*count+1;
    // above can be written as 
    // let Order {ref count, ref item, ..} = food_order;
    println!("{},{:?}", count, item);

    
    let Order {count, item, ..} = food_order;

    // cant use food_order now its moved.
    // println!("{},{:?},{:?}", count, item, food_order);
    println!("{},{:?}", count, item);

    destructure_enum();
    destructure_fn();
    
}
