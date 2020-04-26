use std::env;
use std::fs;

fn main() {
    //grab binary executables environment arguments
    let args: Vec<String> = env::args().collect();
    //only need these two arguments uses references
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    //read specified file, specify error if file cant be read
    let contents = fs::read_to_string(filename)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
