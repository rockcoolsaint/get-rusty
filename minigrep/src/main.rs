use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // replace the &args reference variable with env::args()
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    };
}

// This has been moved to a lib.rs crate file
// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     // let contents = fs::read_to_string(config.filename)
//     //     .expect("something went wrong reading the file");
//     let contents = fs::read_to_string(config.filename)?;
    
//     println!("With text: \n{}", contents);

//     Ok(())
// }
// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();
    
//         Ok(Config {query, filename})
//     }
// }
