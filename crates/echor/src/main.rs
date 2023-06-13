use std::{env, fs};
use clap::{App, Arg};

fn main() {
    println!("NODE_ENV => [{}]", env::var("NODE_ENV").unwrap());

    let matches = App::new("echor")
        .version("0.1.0")
        .author("shijiao")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .allow_invalid_utf8(true)
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();

    let omit_newline = matches.is_present("omit_newline");

    println!("--------------------- rust logic ---------------------");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });

    for name in &text {
        println!("{}", fs::read_to_string("/sandbox/".to_owned() + name).unwrap());
    }
    println!("--------------------- /rust logic ---------------------");
}
