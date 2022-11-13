use std::fs::File;
//use std::io::BufReader;
use std::io::{self, prelude::*, BufReader};
use std::{path, string};
use obj::{load_obj, Obj};
use wgpu_bootstrap::cgmath::{Vector3, Vector2};
use std::path::Path;
use std::ffi::OsStr;
use std::env;
use std::fs;

struct MyFirstVertex {
    position: Vector3<f32>,
    texture: Vector2<f32>,
    normal: Vector3<f32>,
    face: Vector3<Vec<f32>>
}


fn file_reader(path: &str)-> std::io::Result<()>{
    /* 
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            for (index, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                println!("{}. {}", index + 1, line);
            }
        },
        Err(error) => {
            println!("Error opening file {}: {}", path, error);
        },
    }
    */

    let file = File::open(path).expect("file not found");
    let reader = BufReader::new(file);
    for line in reader.lines(){
        match line {
            Ok(line)=>{
                if line.starts_with("v ") {
                    let position_array: [Vector; 1] = [4, 5, 6];
                    println!("{}", line);
                }
            }
            Err(_error)=> {
                println!("fuck");
            }
        }

    }
    Ok(())
}



fn main() {
    // I will implement my main function here
    let path = "src/bunny.obj";
    //println!("{}", file_reader(path));
    file_reader(path);
}