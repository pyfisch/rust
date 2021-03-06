// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/* Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */

#![feature(unboxed_closures)]

use std::mem;
use std::io::stdio::println;

fn call_it<F>(f: F)
    where F : FnOnce(String) -> String
{
    println!("{}", f("Fred".to_string()))
}

fn call_a_thunk<F>(f: F) where F: FnOnce() {
    f();
}

fn call_this<F>(f: F) where F: FnOnce(&str) + Send {
    f("Hello!");
}

fn call_bare(f: fn(&str)) {
    f("Hello world!")
}

fn call_bare_again(f: extern "Rust" fn(&str)) {
    f("Goodbye world!")
}

pub fn main() {
    // Procs

    let greeting = "Hello ".to_string();
    call_it(|s| {
        format!("{}{}", greeting, s)
    });

    let greeting = "Goodbye ".to_string();
    call_it(|s| format!("{}{}", greeting, s));

    let greeting = "How's life, ".to_string();
    call_it(|s: String| -> String {
        format!("{}{}", greeting, s)
    });

    // Closures

    call_a_thunk(|| println!("Hello world!"));

    call_this(|s| println!("{}", s));

    // External functions

    call_bare(println);

    call_bare_again(println);
}

