use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("William Evans <will.evans138@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .conflicts_with("number_nonblank_lines")
                .help("Number the output lines, starting at 1.")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number the non-blank output lines, starting at 1.")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
    	files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => {
                if config.number_lines {
                    for (iterator, line) in open(&filename).unwrap().lines().enumerate() {
                        println!("     {}\t{}", iterator + 1, line.unwrap());
                    }
                }
                else if config.number_nonblank_lines {
                    let mut i = 0;
                    for line in open(&filename).unwrap().lines() {
                        let teststr = line.unwrap();
                        if teststr == "" {
                            println!("");
                        } 
                        else {
                        println!("     {}\t{}", i + 1, teststr);
                        i += 1;
                        }
                    }
                }
                else {
                    for line in open(&filename).unwrap().lines() {
                        println!("{}", line.unwrap());
                    }
                }
            }
        }
    }
    Ok(())
}