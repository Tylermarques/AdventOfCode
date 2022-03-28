use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug)]
struct Point {
    pos: i16,
    depth: i16,
}

fn main() {

    // Collect args and err if not presented with any
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected 1 argument - filename.");
        exit(1)
    }
    let filename = &args[1];

    // Load file to string, split lines into an iter
    let contents = fs::read_to_string(filename).expect("Couldn't read file");
    let file_iter = contents.split("\n");

    // Initialize position vector
    let mut pos_vec = Point { pos: 0, depth: 0 };

    // Go through lines, splitting into instruction and magnitude
    for text in file_iter {
        let instruction_vec = text.split_whitespace().collect::<Vec<&str>>();
        let direction = instruction_vec[0];
        let mag = instruction_vec[1].parse::<i16>().expect("Couldn't parse mag");
        mut_position_vec(&mut pos_vec, direction, mag);
    }

    println!("{:?}", pos_vec)
}

fn mut_position_vec(pos_vec: &mut Point, direction: &str, mag: i16) {
    if direction == "forward" {
        pos_vec.pos = pos_vec.pos + mag
    }
    if direction == "backward" {
        pos_vec.pos = pos_vec.pos - mag
    }
    if direction == "up" {
        pos_vec.depth = pos_vec.depth - mag
    }
    if direction == "down" {
        pos_vec.depth = pos_vec.depth + mag
    }
}