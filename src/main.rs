#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rand;
#[macro_use]
extern crate rocket_contrib;
// #[macro_use]
//extern crate serde_derive;


use serde_json::to_string;
use rocket_contrib::json::JsonValue;
use rand::Rng;
use rocket::response::content;

#[get("/")]
fn pi()->JsonValue{
    json!({
        "sensors": {
          "$ref" : "http://localhost:8000/pi/temperature" //learn using responce in rockt and put this in responce
        },
        "actuators": {
            "$ref" : "http://localhost:8000/pi/light"
        }
    })
}


#[get("/temperature")]
fn temperature()->JsonValue{
    let ran_num = genrate_random();
    json!({
        "Temperature" : ran_num
    })
    
}


#[get("/light")]
fn light()->JsonValue{
    let ran_num = genrate_random();
    json!({
        "Light" : ran_num
    })
}

#[get("/log")]
fn log()->String{
    let values = (temperature(), light());
    format!("{} {}", values.0.to_string(), values.1.to_string())
}

fn main() {
    rocket::ignite()
        .mount("/pi", routes![pi, temperature, light, log])
        // .mount("/", routes![pi])
        // .mount("/temperature", routes![temperature])
        // .mount("/light", routes![light])
        // .mount("/log", routes![log])
        .launch();
}

fn genrate_random()->u32{
    let mut rang = rand::thread_rng();
    let num = rang.gen_range(0,100);
    num
}