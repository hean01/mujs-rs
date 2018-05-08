use std::env;
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;

extern crate mujs;

pub fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: jsrun <filename>");
        exit(1);
    }

    let filename = &args[1];

    // read file into string
    let mut f = File::open(filename)
        .expect("file not found");

    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("failed to read content of file");

    // run string
    let state = mujs::State::new(mujs::JS_STRICT);
    state.loadstring(filename, content.as_str())
        .expect("failed to parse javascript");

    // run program
    state.newobject();
    match state.call(0) {
        Ok(_) => {
            let res = state.tostring(-1)
                .expect("no result on stack");

            println!("Result: {}", res)
        },
        Err(e) => println!("Failed call: {}", e)
    }

    exit(0);
}
