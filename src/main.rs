use std::env;
use std::process;

use mini_grep::Config;

fn main() {
  let args: Vec<String> = env::args().collect();

  // dbg!(args);
  // println!("searching for {}", query);
  // println!("In file {}", file_path);
  let config = Config::build(&args).unwrap_or_else(|err| {
    // println!("Problem parsing arguments: {err}");
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
  });

  if let Err(e) = mini_grep::run(config) {
    // println!("Application error: {e}");
    eprintln!("Application error: {e}");

    process::exit(1);
  }
}

// fn parse_config(args: &[String]) -> Config {
//   let query = args[1].clone();
//   let file_path = args[2].clone();

//   Config { query, file_path }
// }
