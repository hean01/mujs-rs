extern crate libc;


#[link(name = "mujs", kind="static")]
use std::ffi::CStr;


use libc::{
    c_int,
    c_double,
    c_void,
    c_char
};



extern {
    fn js_newstate(alloc: *const c_void, context: *const c_void, flags: c_int) -> *const c_void;
    fn js_freestate(J: *const c_void);
    fn js_gc(J: *const c_void, report: c_int);
    fn js_ploadstring(J: *const c_void, filename: *const c_char, source: *const c_char) -> c_int;
    fn js_pcall(J: *const c_void, n: c_int) -> c_int;
    fn js_dostring(J: *const c_void, source: *const c_char) -> c_int;

    fn js_newobject(J: *const c_void);

    fn js_isobject(J: *const c_void, idx: c_int) -> c_int;

    fn js_hasproperty(J: *const c_void, idx: c_int, name: *const c_char) -> c_int;
    fn js_getproperty(J: *const c_void, idx: c_int, name: *const c_char);
    fn js_setproperty(J: *const c_void, idx: c_int, name: *const c_char);

    fn js_pushundefined(J: *const c_void);
    fn js_pushnull(J: *const c_void);
    fn js_pushboolean(J: *const c_void, v: c_int);
    fn js_pushnumber(J: *const c_void, v: c_double);

    fn js_isdefined(J: *const c_void, idx: c_int) -> c_int;
    fn js_isundefined(J: *const c_void, idx: c_int) -> c_int;

    fn js_throw(J: *const c_void);

    fn js_newerror(J: *const c_void, message: *const c_char);
    fn js_newevalerror(J: *const c_void, message: *const c_char);
    fn js_newrangeerror(J: *const c_void, message: *const c_char);
    fn js_newreferenceerror(J: *const c_void, message: *const c_char);
    fn js_newsyntaxerror(J: *const c_void, message: *const c_char);
    fn js_newtypeerror(J: *const c_void, message: *const c_char);
    fn js_newurierror(J: *const c_void, message: *const c_char);

    fn js_tostring(J: *const c_void, idx: i32) -> *const c_char;
    fn js_toboolean(J: *const c_void, idx: i32) -> c_int;
    fn js_tonumber(J: *const c_void, idx: i32) -> c_double;
}

pub struct State {
    state: *const c_void,
    memctx: *const c_void,
}

impl State {

    pub fn new() -> State {
        let mut js = State {
            state: std::ptr::null(),
            memctx: std::ptr::null(),
        };

        let js_ptr: *const State = &js;
        js.memctx = js_ptr as *const c_void;
        js.state = unsafe { js_newstate(std::ptr::null(), js.memctx, 0) };

        js
    }

    pub fn gc(self: &State, report: bool) {
        match report {
            true => unsafe { js_gc(self.state, 1) },
            false => unsafe { js_gc(self.state, 0) }
        }
    }

    pub fn loadstring(self: &State, filename: &str, source: &str) -> Result<(), String> {
        match unsafe { js_ploadstring(self.state, filename.as_ptr() as *const c_char, source.as_ptr() as *const c_char) } {
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
        match unsafe {js_dostring(self.state, source.as_ptr() as *const c_char) } {
            0 => Ok(()),
            _ => Err("Failed to run script".to_string())
        }
    }

    pub fn throw(self: &State) {
        unsafe { js_throw(self.state) };
    }

    pub fn newerror(self: &State, message: &str) {
        unsafe { js_newerror(self.state, message.as_ptr() as *const c_char) };
    }

    pub fn newevalerror(self: &State, message: &str) {
        unsafe { js_newevalerror(self.state, message.as_ptr() as *const c_char) }
    }

    pub fn newrangeerror(self: &State, message: &str) {
        unsafe { js_newrangeerror(self.state, message.as_ptr() as *const c_char) }
    }

    pub fn newreferenceerror(self: &State, message: &str) {
        unsafe { js_newreferenceerror(self.state, message.as_ptr() as *const c_char) }
    }

    pub fn newsyntaxerror(self: &State, message: &str) {
        unsafe { js_newsyntaxerror(self.state, message.as_ptr() as *const c_char) }
    }

    pub fn newtypeerror(self: &State, message: &str) {
        unsafe { js_newtypeerror(self.state, message.as_ptr() as *const c_char) }
    }

    pub fn newurierror(self: &State, message: &str) {
        unsafe { js_newurierror(self.state, message.as_ptr() as *const c_char) }
    }

    pub fn newobject(self: &State) {
        unsafe { js_newobject(self.state) };
    }

    pub fn isobject(self: &State, idx: i32) -> bool {
        match unsafe { js_isobject(self.state, idx) } {
            0 => false,
            _ => true
        }
    }

    pub fn pushundefined(self: &State) {
        unsafe { js_pushundefined(self.state) };
    }

    pub fn pushnull(self: &State) {
        unsafe { js_pushnull(self.state) };
    }

    pub fn pushboolean(self: &State, value: bool) {
        match value {
            false => unsafe { js_pushboolean(self.state, 0) },
            true => unsafe { js_pushboolean(self.state, 1) }
        }
    }


    pub fn pushnumber(self: &State, value: f64) {
        unsafe { js_pushnumber(self.state, value) }
    }

    pub fn hasproperty(self: &State, idx: i32, name: String) -> bool {
        match unsafe { js_hasproperty(self.state, idx, name.as_ptr() as *const c_char) } {
            0 => false,
            _ => true
        }
    }

    pub fn setproperty(self: &State, idx: i32, name: String) {
        unsafe { js_setproperty(self.state, idx, name.as_ptr() as *const c_char) };
    }

    pub fn getproperty(self: &State, idx: i32, name: String) {
        unsafe { js_getproperty(self.state, idx, name.as_ptr() as *const c_char) };
    }

    pub fn isdefined(self: &State, idx: i32) -> bool {
        match unsafe { js_isdefined(self.state, idx) } {
            0 => false,
            _ => true
        }
    }

    pub fn isundefined(self: &State, idx: i32) -> bool {
        match unsafe { js_isundefined(self.state, idx) } {
            0 => false,
            _ => true
        }
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

    pub fn toboolean(self: &State, idx: i32) -> Result<bool, String> {
        match unsafe { js_toboolean(self.state, idx) } {
            0 => Ok(false),
            _ => Ok(true)
        }
    }

    pub fn tonumber(self: &State, idx: i32) -> Result<f64, String> {
        Ok( unsafe { js_tonumber(self.state, idx) } )
    }

}

impl Drop for State {
    fn drop(self: &mut State) {
        unsafe { js_freestate(self.state) };
    }
}


#[cfg(test)]
mod tests {
    use std;
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
        assert_eq!(state.tostring(0).ok().unwrap(), "Hello World!");
    }

    #[test]
    fn tostring_utf8() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "'Hello' + ' ' + 'Båsse!';").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.tostring(0).ok().unwrap(), "Hello Båsse!");
    }

    #[test]
    fn tostring_with_number_on_stack() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "240.32;").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.tostring(0).ok().unwrap(), "240.32");
    }

    #[test]
    fn toboolean_with_true_on_stack() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "true").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.toboolean(0).ok().unwrap(), true);
    }

    #[test]
    fn toboolean_with_false_on_stack() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "false").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.toboolean(0).ok().unwrap(), false);
    }

    #[test]
    fn toboolean_with_positive_number_on_stack() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "1").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.toboolean(0).ok().unwrap(), true);
    }

    #[test]
    fn toboolean_with_zero_number_on_stack() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "0").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.toboolean(0).ok().unwrap(), false);
    }

    #[test]
    fn toboolean_with_null_on_stack() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "null").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.toboolean(0).ok().unwrap(), false);
    }

    #[test]
    fn toboolean_with_undefined_on_stack() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "undefined").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.toboolean(0).ok().unwrap(), false);
    }

    #[test]
    fn tonumber_with_positive_number_on_stack() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "1.53278").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.tonumber(0).ok().unwrap(), 1.53278);
    }

    #[test]
    fn tonumber_with_negative_number_on_stack() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "-1.53278;").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.tonumber(0).ok().unwrap(), -1.53278);
    }

    #[test]
    fn tonumber_with_valid_string_on_stack() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "'1.53278'").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.tonumber(0).ok().unwrap(), 1.53278);
    }

    #[test]
    fn tonumber_with_invalid_string_on_stack() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "'hello world'").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.tonumber(0).ok().unwrap().classify(), std::num::FpCategory::Nan);
    }

    #[test]
    fn pushundefined_verify_as_string() {
        let state = ::State::new();
        state.pushundefined();
        assert_eq!(state.tostring(0).ok().unwrap(), "undefined");
     }

    #[test]
    fn pushnull_verify_as_string() {
        let state = ::State::new();
        state.pushnull();
        assert_eq!(state.tostring(0).ok().unwrap(), "null");
    }

    #[test]
    fn pushboolean_verify_as_string() {
        let state = ::State::new();
        state.pushboolean(true);
        assert_eq!(state.tostring(0).ok().unwrap(), "true");
    }

    #[test]
    fn pushnumber_verify_as_string() {
        let state = ::State::new();
        state.pushnumber(1.234);
        assert_eq!(state.tostring(0).ok().unwrap(), "1.234");
    }

    #[test]
    fn newerror_verify_as_string() {
        let state = ::State::new();
        state.newerror("This is an error");
        assert_eq!(state.tostring(0).ok().unwrap(), "Error: This is an error");
    }

    #[test]
    fn newevalerror_verify_as_string() {
        let state = ::State::new();
        state.newevalerror("This is an error");
        assert_eq!(state.tostring(0).ok().unwrap(), "EvalError: This is an error");
    }

    #[test]
    fn newrangeerror_verify_as_string() {
        let state = ::State::new();
        state.newrangeerror("This is an error");
        assert_eq!(state.tostring(0).ok().unwrap(), "RangeError: This is an error");
    }

    #[test]
    fn newreferenceerror_verify_as_string() {
        let state = ::State::new();
        state.newreferenceerror("This is an error");
        assert_eq!(state.tostring(0).ok().unwrap(), "ReferenceError: This is an error");
    }

    #[test]
    fn newsyntaxerror_verify_as_string() {
        let state = ::State::new();
        state.newsyntaxerror("This is an error");
        assert_eq!(state.tostring(0).ok().unwrap(), "SyntaxError: This is an error");
    }

    #[test]
    fn newtypeerror_verify_as_string() {
        let state = ::State::new();
        state.newtypeerror("This is an error");
        assert_eq!(state.tostring(0).ok().unwrap(), "TypeError: This is an error");
    }

    #[test]
    fn newurierror_verify_as_string() {
        let state = ::State::new();
        state.newurierror("This is an error");
        assert_eq!(state.tostring(0).ok().unwrap(), "URIError: This is an error");
    }

    #[test]
    fn isdefined_on_undefined_is_false() {
        let state = ::State::new();
        state.pushundefined();
        assert_eq!(state.isdefined(0), false);
    }

    #[test]
    fn isdefined_on_number_is_true() {
        let state = ::State::new();
        state.pushnumber(1.234);
        assert_eq!(state.isdefined(0), true);
    }

    #[test]
    fn isundefined_on_undefined_is_true() {
        let state = ::State::new();
        state.pushundefined();
        assert_eq!(state.isundefined(0), true);
    }

    #[test]
    fn isundefined_on_number_is_false() {
        let state = ::State::new();
        state.pushnumber(1.234);
        assert_eq!(state.isundefined(0), false);
    }

    #[test]
    fn isobject_on_object_is_true() {
        let state = ::State::new();
        state.newobject();
        assert_eq!(state.isobject(0), true);
    }

    #[test]
    fn isobject_on_number_is_false() {
        let state = ::State::new();
        state.pushnumber(1.234);
        assert_eq!(state.isobject(0), false);
    }

    #[test]
    fn hasproperty_on_object_with_existing_property() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "var person = {name: \"Tester\", age: 32}; person").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.hasproperty(0, "age".to_string()), true);
    }

    #[test]
    fn hasproperty_on_object_with_non_existing_property() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "var person = {name: \"Tester\", age: 32}; person").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        assert_eq!(state.hasproperty(0, "phone".to_string()), false);
    }

    #[test]
    fn getproperty_on_object_with_existing_property() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "var person = {name: \"Tester\", age: 32}; person").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        state.getproperty(0, "name".to_string());
        assert_eq!(state.tostring(1).ok().unwrap(), "Tester");
    }

    #[test]
    fn getproperty_on_object_with_non_existing_property() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "var person = {name: \"Tester\", age: 32}; person").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        state.getproperty(0, "phone".to_string());
        assert_eq!(state.isundefined(1), true);
    }

    #[test]
    fn setproperty_on_object_as_number_value() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "var person = {name: \"Tester\", age: 32}; person").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        state.pushnumber(1.234);
        state.setproperty(0, "age".to_string());
        state.getproperty(0, "age".to_string());
        assert_eq!(state.tonumber(1).unwrap(), 1.234);
    }

    #[test]
    fn setproperty_on_object_changing_to_number_value() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "var person = {name: \"Tester\", age: 32}; person").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        state.pushnumber(1.234);
        state.setproperty(0, "name".to_string());
        state.getproperty(0, "name".to_string());
        assert_eq!(state.tonumber(1).unwrap(), 1.234);
    }


    #[test]
    fn setproperty_on_object_non_existing_property() {
        let state = ::State::new();
        assert!(state.loadstring("myscript", "var person = {name: \"Tester\", age: 32}; person").is_ok());
        state.newobject();
        assert!(state.call(0).is_ok());
        state.pushnumber(1.234);
        state.setproperty(0, "phone".to_string());
        state.getproperty(0, "phone".to_string());
        assert_eq!(state.tonumber(1).unwrap(), 1.234);
    }

}
