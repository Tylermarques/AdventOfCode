use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file");
    let mut old_int: i16 = 0;
    for text in contents.split("\n") {
        let my_int = text.parse::<i16>().expect("Wanted an int, got other");
        
        if my_int > old_int{
            println!("increasing")
        };
        old_int = my_int;
    }
}
