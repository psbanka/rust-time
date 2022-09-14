use std::fs::File;
// use std::io::{prelude::*, BufReader};
use std::io::{BufRead, BufReader};
use url::Url;

fn parse_url(url: String) -> Option<Url> {
    match Url::parse(url.as_str()) {
        Ok(url) => Some(url),
        Err(_) => None,
    }
}

fn main() {
    let f = File::open("src/data/content.txt");

    match f {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut linenos = 0;

            for line in reader.lines() {
                match line {
                    Ok(linestr) => {
                        linenos += 1;
                        match parse_url(linestr) {
                            Some(url) => println!("{}", url),
                            None => {}
                        }
                    }
                    Err(e) => panic!("Error: {}", e),
                }
            }

            println!("The number of lines in the file is {linenos}");
        }
        Err(e) => panic!("Error: {}", e),
    };
}

#[test]
fn test_parse_url_happy() {
    let url = "https://www.rust-lang.org".to_string();
    let parsed_url = parse_url(url);
    assert_eq!(parsed_url.is_some(), true);
    assert_eq!(parsed_url.unwrap().host_str().unwrap(), "www.rust-lang.org");
}

#[test]
fn test_parse_url_unhappy() {
    let url = "garbage/junk.com".to_string();
    let parsed_url = parse_url(url);
    assert_eq!(parsed_url.is_none(), true);
}
