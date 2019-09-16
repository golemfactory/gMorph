use std::fs::File;
use std::io::prelude::*;

fn main() {
    //    let x = vec!(1,2,3,4);
    //    let y = vec!(2,4,6,8);

    let mut x = Vec::new();
    let mut y = Vec::new();

    for i in 1..1000 {
        x.push(i);
        y.push((2.71*(i as f64)).round() as u32);
    }
    let serialized = serde_json::to_string(&(x,y))
        .expect("Failed to serialize vectors");
    let mut data_file = File::create("input.json")
        .expect("Failed to create input.json");
    data_file
        .write_all(serialized.as_bytes())
        .expect("Failed to write input.json");

}
