use std::process;

#[macro_use]
extern crate clap;

mod days;

fn main() {
    let matches = clap_app!(app =>
        (version: "1.0")
        (author: "borfus")
        (about: "Advent of Code 2021")
        (@arg DAY: -d +takes_value --day +takes_value +required "Challenge day number")
    ).get_matches();

    let day = matches.value_of("DAY");
    let day = day.unwrap().parse::<i32>();
    let day = match day {
        Ok(d) => d,
        Err(_) => {
            println!("Can't parse day arg!");
            process::exit(1);
        }
    };

    days::run(day);
}

