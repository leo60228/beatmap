use std::io::{stdin, stdout};
use std::env::args;
use std::process::exit;
use serde_json::{from_reader, to_writer_pretty};
use beatmap::*;

fn main() {
    let mode = args().nth(1).expect("Must provide mode!");

    if mode == "to_json" {
        to_writer_pretty(stdout(), &Beatmap::read(&mut stdin()).unwrap()).unwrap();
    } else if mode == "from_json" {
        from_reader::<_, Beatmap>(stdin()).unwrap().write(&mut stdout()).unwrap();
    } else {
        eprintln!("Mode must be to_json or from_json!");
        exit(1);
    }
}
