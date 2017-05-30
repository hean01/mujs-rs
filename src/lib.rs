extern crate libc;


#[link(name = "mujs", kind="static")]
use std::ffi::CStr;


use libc::{
    c_int,
    c_void,
    c_char
};



extern {
    fn js_newstate(alloc: *const c_void, context: *const c_void, flags: c_int) -> *const c_void;
    fn js_freestate(J: *const c_void);
    fn js_gc(J: *const c_void, report: c_int);
    fn js_ploadstring(J: *const c_void, filename: *const u8, source: *const u8) -> c_int;
    fn js_pcall(J: *const c_void, n: c_int) -> c_int;
    fn js_dostring(J: *const c_void, source: *const c_char) -> c_int;
    fn js_newobject(J: *const c_void);
    fn js_tostring(J: *const c_void, idx: i32) -> *const c_char;
}

pub struct State {
    state: *const c_void, 
}

impl State {

    pub fn new() -> State {
        State {
            state: unsafe { js_newstate(std::ptr::null(), std::ptr::null(), 0) }
        }
    }

    pub fn gc(self: &State, report: bool) {
        match report {
            true => unsafe { js_gc(self.state, 1) },
            false => unsafe { js_gc(self.state, 0) }
        }
    }

    pub fn loadstring(self: &State, filename: &str, source: &str) -> Result<(), String> {
        match unsafe { js_ploadstring(self.state, filename.as_ptr(), source.as_ptr()) } {
            0 => Ok(()),
            _ => {
                let err = self.tostring(-1);
                assert!(err.is_ok());
                Err(err.ok().unwrap())
            }
        }
    }

    pub fn call(self: &State, n: i32) -> Result<(), String> {
        match unsafe { js_pcall(self.state, n) } {
            0 => Ok(()),
            _ => {
                let err = self.tostring(-1);
                assert!(err.is_ok());
                Err(err.ok().unwrap())
            }
        }
    }

    pub fn dostring(self: &State, source: &str) -> Result<(), String> {
        match unsafe {js_dostring(self.state, source.as_ptr() as *const i8) } {
            0 => Ok(()),
            _ => Err("Failed to run script".to_string())
        }
    }

    pub fn newobject(self: &State) {
        unsafe { js_newobject(self.state) };
    }

    pub fn tostring(self: &State, idx: i32) -> Result<String, String> {
        let c_buf: *const c_char = unsafe { js_tostring(self.state, idx) };

        if c_buf == std::ptr::null() {
            return Err("Null string".to_string())
        }

        Ok(unsafe {
            CStr::from_ptr(c_buf).to_string_lossy().into_owned()
        })
    }

}

impl Drop for State {
    fn drop(self: &mut State) {
        unsafe { js_freestate(self.state) };
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn create_new_state() {
        let _ = ::State::new();
    }

    #[test]
    fn call_garbage_collector() {
        let state = ::State::new();
        state.gc(false);
    }

    #[test]
    fn loadstring_with_broken_script() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "func broken() {").is_err());
    }

    #[test]
    fn loadstring_with_complete_script() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "func broken() { return Math.sin(3.2); };").is_err());
    }

    #[test]
    fn call_with_runtime_error() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "mystic.func();").is_ok());
        state.newobject();
        assert!(state.call(0).is_err());
    }

    #[test]
    fn call_with_success() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "Math.sin(3.2);").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
    }

    #[test]
    fn dostring_with_success() {
        let state = ::State::new();
        assert!(state.dostring("Math.sin(3.2);").is_ok());
    }

    #[test]
    fn dostring_with_broken_script() {
        let state = ::State::new();
        assert!(state.dostring("func broken() {").is_err());
    }

    #[test]
    fn dostring_with_runtime_error() {
        let state = ::State::new();
        assert!(state.dostring("mystic.func();").is_err());
    }

    #[test]
    fn tostring_ascii() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "'Hello' + ' ' + 'World!';").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert!(state.tostring(0).ok().unwrap() == "Hello World!");
    }

    #[test]
    fn tostring_utf8() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "'Hello' + ' ' + 'Båsse!';").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert!(state.tostring(0).ok().unwrap() == "Hello Båsse!");
    }

    #[test]
    fn tostring_with_number_on_stack() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "240.32;").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert!(state.tostring(0).ok().unwrap() == "240.32");
    }
}
