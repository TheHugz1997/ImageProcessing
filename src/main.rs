use std::fs::File;
//use std::io::BufReader;
use std::io::{self, prelude::*, BufReader};
use std::path;
use obj::{load_obj, Obj};
use std::path::Path;
use std::ffi::OsStr;
use std::env;
use std::fs;


fn file_reader(path: &str)-> std::io::Result<()>{
    /*
    let data = fs::read_to_string(path).expect("Unable to read file");
    return data
    */
    let file = File::open(path).expect("file not found");
    let reader = BufReader::new(file);

    for line in reader.lines(){
        println!("{}", line?);
    }
    Ok(())
}

struct MyFirstVertex {
    position: Vec<f32>,
    texture: Vec<f32>,
    normal: Vec<f32>
}

fn main() {
    // I will implement my main function here
    let path = "src/bunny.obj";
    //println!("{}", file_reader(path));
    file_reader(path);
}