use std::env;
use std::fs;
use std::process;

fn main() {
    //grab binary executables environment arguments
    let args: Vec<String> = env::args().collect();

    //only need these two arguments uses references
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);


    run(config);
    
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //read specified file, specify error if file cant be read
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
