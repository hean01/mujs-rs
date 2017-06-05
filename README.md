# mujs-rs - rust MuJS bindings
[![Version](https://img.shields.io/crates/v/mujs.svg)](https://crates.io/crates/mujs)
[![License](https://img.shields.io/crates/l/mujs.svg)](https://crates.io/crates/mujs)
[![Downloads](https://img.shields.io/crates/d/mujs.svg)](https://crates.io/crates/mujs)
[![Status](https://img.shields.io/travis/hean01/mujs-rs/master.svg)](https://travis-ci.org/hean01/mujs-rs)

[MuJS](http://mujs.com) is a lightweight implementation of the
Javascript language in a library. MuJS is licensed under AGPL and
so is this rust bindings.

Its primary purpose and design is for embedding in other software
to add scripting capability to those programs, but it can also be
used as an extensible scripting language.

In contrast to other programs that are large and complex, MuJS was
designed with a focus on small size, correctness and
simplicity. MuJS is written in portable C and implements
ECMAScript as specified by ECMA-262.

The interface for binding with native code is designed to be as
simple as possible to use, and is similar to Lua.
