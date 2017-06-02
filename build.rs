extern crate gcc;

fn main() {
    gcc::compile_library("libmujs.a", &["mujs/one.c"]);
}
