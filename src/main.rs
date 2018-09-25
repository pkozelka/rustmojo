#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::path::Path;
use serde_json::Value;

fn tocsv<P: AsRef<Path>>(path: P) {
    let file = File::open (path).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).expect("JSON was not well-formatted");
    match json {
        Value::Array(array) => for item in array {
            println!("item={}", item);
        },
        Value::Object(map) => for key in map.keys() {
            println!("key={} value={}", key, map.get(key).unwrap());
        },
        _ => println!("other")
    };
}

fn main() {
    let mut args = env::args();
    let myself = args.next(); // args[0] is the executable file name
    println!("Executing {}:", myself.unwrap());
    let files: Vec<String> = args.collect();
    if files.is_empty() {
        eprintln!("ERROR: At least one file is required!");
        std::process::exit(1);
    }
    for filename in files {
        println!("reading file: {}", &filename);
        let file = File::open(&filename.as_str()).unwrap();
        let json: serde_json::Value = serde_json::from_reader(file).expect("JSON was not well-formatted");
        println!("Json: {}", json);

        tocsv(&filename);

        let req = xxx::read_request(&filename).unwrap();
        for flight in req.flights {
            println!("{} {:0>4} [{:>4}]: {} --> {} / {}  {} ({})",
                     flight.departure_date,
                     flight.flight_number,
                     flight.agency_code,
                     flight.departure,
                     flight.arrival,
                     flight.country,
                     flight.last_fare_class_seat_availability_map,
                     flight.fare_class_mean,
            )
        }
    }
}
