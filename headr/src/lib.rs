use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
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
                .multiple(true),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("Number of lines")
                .default_value("10")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("number_bytes")
                .short("c")
                .help("Number of bytes")
                .takes_value(true),
        )
        .get_matches();

    if matches.occurrences_of("number_lines") > 0 {
        if matches.occurrences_of("number_bytes") > 0 {
            panic!(
                "The argument '--lines <LINES>' cannot be \
               used with '--bytes <BYTES>'"
            )
        }
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: match parse_positive_int(&matches.values_of_lossy("number_lines").unwrap().join(""))
        {
            Ok(n) => n,
            Err(e) => {
                panic!("illegal line count -- {}", e);
            }
        },
        bytes: {
            match matches.values_of_lossy("number_bytes") {
                None => None,
                Some(_n) => Some(
                    match parse_positive_int(
                        &matches.values_of_lossy("number_bytes").unwrap().join(""),
                    ) {
                        Ok(n) => n,
                        Err(e) => {
                            panic!("illegal byte count -- {}", e);
                        }
                    },
                ),
            }
        },
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
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
