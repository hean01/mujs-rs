extern crate gcc;

fn main() {
    gcc::compile_library("libmujs.a", &["mujs/one.c"]);
        /*
        .file("mujs/jsarray.c")
        .file("mujs/jsboolean.c")
        .file("mujs/jsbuiltin.c")
        .file("mujs/jscompile.c")
        .file("mujs/jsdate.c")
        .file("mujs/jsdtoa.c")
        .file("mujs/jsdump.c")
        .file("mujs/jserror.c")
        .file("mujs/jsfunction.c")
        .file("mujs/jsgc.c")
        .file("mujs/jsintern.c")
        .file("mujs/jslex.c")
        .file("mujs/jsmath.c")
        .file("mujs/jsnumber.c")
        .file("mujs/jsobject.c")
        .file("mujs/json.c")
        .file("mujs/jsparse.c")
        .file("mujs/jsproperty.c")
        .file("mujs/jsrexexp.c")
        .file("mujs/jsrun.c")
        .file("mujs/jsstate.c")
        .file("mujs/jsstring.c")
        .file("mujs/jsvalue.c")
        .file("mujs/")
     */

}
