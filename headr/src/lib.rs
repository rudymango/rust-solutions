use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    // lines: usize,
    // bytes: Option<usize>,
}
// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("William Evans <will.evans138@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
            )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("Displays files with certain amount of lines")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("number_bytes")
                .short("c")
                .help("Displays files with certain amount of bytes")
                .takes_value(true)
                .conflicts_with("number_lines")
        )
        .get_matches();

        Ok(Config{
            files: matches.values_of_lossy("files").unwrap(),
            // lines: ...
            // bytes: ...
        })
}


fn parse_positive_int(val: &str) -> MyResult<usize> {
    let usize_zero: usize = 0; 
    if val.parse::<usize>().is_ok() {
        if val.parse::<usize>().unwrap() == usize_zero {
            Err(Box::<dyn Error>::from(val))
        } else {
        Ok(val.parse::<usize>()?) 
        }
    } else {
        Err(Box::<dyn Error>::from(val))
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());

}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}