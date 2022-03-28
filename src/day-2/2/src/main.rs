use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug)]
struct Point {
    pos: i64,
    depth: i64,
    aim: i64,
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
    let mut pos_vec = Point { pos: 0, depth: 0, aim: 0};

    // Go through lines, splitting into instruction and magnitude
    for text in file_iter {
        if text == "" {
            continue;
        }
        let instruction_vec = text.split_whitespace().collect::<Vec<&str>>();
        let direction = instruction_vec[0];
        let mag = instruction_vec[1].parse::<i64>().expect("Couldn't parse mag");
        mut_position_vec(&mut pos_vec, direction, mag);
        if pos_vec.pos <= 0 {
            println!("Less than 0");
            println!("{:?}", pos_vec);
        }
    }

    println!("{:?}", pos_vec);
    println!("{}",pos_vec.pos*pos_vec.depth)

}

fn mut_position_vec(pos_vec: &mut Point, direction: &str, mag: i64) {
    if direction == "forward" {
        pos_vec.pos = pos_vec.pos + mag;
        pos_vec.depth = pos_vec.depth + (pos_vec.aim*mag);
    }
    if direction == "up" {
        pos_vec.aim = pos_vec.aim - mag;
    }
    if direction == "down" {
        pos_vec.aim = pos_vec.aim + mag
    }
}
