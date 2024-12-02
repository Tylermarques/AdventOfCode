use itermore::IterMore;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file");
    let file_iter = contents.split("\n");
    let mut int_vect = Vec::new();
    for text in file_iter {
        int_vect.push(text.parse::<i16>().expect("Wanted an int, got other"));
    }
    let mut window = int_vect.iter().copied().windows::<3>();

    let mut old_int: i16 = (window.next().unwrap().iter().sum::<i16>()) / 3;
    let mut new_int: i16 = (window.next().unwrap().iter().sum::<i16>()) / 3;
    if new_int > old_int {
        println!("increasing")
    };
    for cur_window in window {
        new_int = cur_window.iter().sum();
        if new_int > old_int {
            println!("increasing")
        };
        old_int = new_int;
    }
}
