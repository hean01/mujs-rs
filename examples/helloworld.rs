extern crate mujs;

pub fn main() {
    let state = mujs::State::new();
    assert!(state.loadstring("myscript", "'hello' + ' ' + 'world';").is_ok());
    state.newobject();

    match state.call(0) {
        Ok(_) => {
            let res = state.tostring(-1).ok().unwrap();
            println!("Result: {}", res)
        },
        Err(e) => println!("Failed call: {}", e)
    }
}
