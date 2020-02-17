//! bare3.rs
//!
//! String types in Rust
//!
//! What it covers:
//! - Types, str, arrays ([u8;uszie]), slices (&[u8])
//! - Iteration, copy
//! - Semihosting (tracing)

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("bare3").unwrap();
    let s = "ABCD";
    let bs = s.as_bytes();

    hprintln!("s = {}", s).unwrap();
    hprintln!("bs = {:?}", bs).unwrap();

    hprintln!("iterate over slice").unwrap();
    for c in bs {
        hprint!("{},", c).unwrap();
    }

    hprintln!("iterate iterate using (raw) indexing").unwrap();
    for i in 0..s.len() {
        hprintln!("{},", bs[i]).unwrap();
    }

    hprintln!("").unwrap();

    let a = [65u8; 4];
    // let mut a = [0u8; 4];

    hprintln!("").unwrap();
    hprintln!("a = {}", core::str::from_utf8(&a).unwrap()).unwrap();

    loop {
        continue;
    }
}

// 0. Build and run the application (debug build).
//
//    > cargo run --example bare3
//    (or use the vscode build task)
//
// 1. What is the output in the `openocd` (Adapter Output) console?
//
//    ** your answer here **
//
//    What is the type of `s`?
//
//    ** your answer here **
//
//    What is the type of `bs`?
//
//    ** your answer here **
//
//    What is the type of `c`?
//
//    ** your answer here **
//
//    What is the type of `a`?
//
//    ** your answer here **
//
//    What is the type of `i`?
//
//    ** your answer here **
//
//    Commit your answers (bare3_1)
//
// 2. Make types of `s`, `bs`, `c`, `a`, `i` explicit.
//
//    Commit your answers (bare3_2)
//
// 3. Uncomment line `let mut a = [0u8; 4];
//`
//    Run the program, what happens and why?
//
//    ** your answer here **
//
//    Commit your answers (bare3_3)
//
// 4. Alter the program so that the data from `bs` is copied byte by byte into `a`.
//
//    Test that it works as intended.
//
//    Commit your answers (bare3_4)
//
// 5. Look for a way to make this copy done without a loop.
//    https://doc.rust-lang.org/std/primitive.slice.html
//
//    Implement and test your solution.
//
//    Commit your answers (bare3_5)
