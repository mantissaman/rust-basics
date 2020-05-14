use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use std::thread;
use std::time;

pub fn run_simulation(){

    //1. Store State - location, velocity and acceleration
    let mut location: f64 = 0.0; //meters
    let mut velocity: f64 = 0.0; //meters per second
    let mut acceleration: f64 = 0.0; //meters per second squared

    //2. Store Motor Input
    let mut up_input_voltage: f64 = 0.0; //volts
    let mut down_input_voltage: f64 =0.0; //volts

    //3. Store Input Building Description and Requests
    let mut floor_count: u64 = 0;
    let mut floor_height: f64 = 0.0; //meters
    let mut floor_requests: Vec<u64> = Vec::new();

    //4.  Parse input and store Building Description and Requests
    let buffer = match env::args().nth(1) {
        Some(ref fp) if *fp == "-".to_string() => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        },
        None => {
            let fp = "test1.txt";
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open failed")
                .read_to_string(&mut buffer)
                .expect("read_to_string_failed");
            buffer
        },
        Some(fp) => {
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open failed")
                .read_to_string(&mut buffer)
                .expect("read_to_string_failed");
            buffer           
        }
    };


    for (li, l) in buffer.lines().enumerate() {
        if li == 0 {
            floor_count =l.parse::<u64>().unwrap();
        } else if li == 1 {
            floor_height =l.parse::<f64>().unwrap();
        } else {
            floor_requests.push(l.parse::<u64>().unwrap());
        }

    }

    //Store timestamp
    let mut prev_loop_time = Instant::now();
    //5. Loop while floor requests remain
    while floor_requests.len() > 0 {
        //5.1 Update State - location, velocity and acceleration
        let now = Instant::now();
        let dt = now.duration_since(prev_loop_time)
            .as_secs_f64();
        prev_loop_time = now;

        location = location + velocity * dt;
        velocity = velocity + acceleration * dt;
        // a = F/m
        acceleration = {
            // DC motor creates 8 Newtons foce per volts
            let F = (up_input_voltage - down_input_voltage) * 8.0;
            
            //Weight of elevator carriage is 1,200 KG
            let m = 1200000.0;

            // 9.8 meters per second sqaured is g
            -9.8 + F/m

        };
        

        //5.2 If next floor request is satisfied then remove from queue
        let next_floor = floor_requests [0];
        if (location - (next_floor as f64) * floor_height ).abs() < 0.01
            && velocity.abs() < 0.01 {
                velocity = 0.0;
                floor_requests.remove(0);
        }

        //5.3 Adjust motor control to process next floor request
        // max acceleration 1 meter per second squared (1.5 slack)
        // max velocity 5 meter per second (> 5 is unsafe)

        // first think about decelaration
        // it takes t seconds to decelarate from velocity v @ 1m/s*s
        let t = velocity.abs() / 1.0;
        // during decelaration we travel t * v/2 meters
        let d = t * (velocity / 2.0);
        // l is distance to next floor
        let l = (location - (next_floor as f64) * floor_height).abs();        

        //Three cases to consider
        //1. if we are in decelaration range - we should slow down
        //2. if not in decelaration range and not max velocity then accelerate
        //3. if not in decelaration range and max velacity - no velocity change

        let target_acceleration = {
            //are we going up?
            let going_up = location < (next_floor as f64) * floor_height;

            //Do not exceed velocity
            // if velocity.abs() >=5.0 {
            //     if (going_up && v)
            // }

        };

        //5.4 Print realtime stats

        //Sleep a little
        thread::sleep(time::Duration::from_millis(10));
    }

    //6. Print Summary
    println!("Summary");

}

fn main() {
    run_simulation();
}