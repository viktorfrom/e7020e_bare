//! bare3.rs
//!
//! String types in Rust
//!
//! What it covers:
//! - Types, str, arrays ([u8; usize]), slices (&[u8])
//! - Iteration, copy
//! - Semihosting (tracing using `hprintln`

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("bare3").unwrap();
    let s: &str = "ABCD";
    let bs: &[u8] = s.as_bytes();

    hprintln!("s = {}", s).unwrap();
    hprintln!("bs = {:?}", bs).unwrap();

    hprintln!("iterate over slice").unwrap();
    for c in bs {
        hprint!("{},", c as &u8).unwrap();
    }

    hprintln!("iterate iterate using (raw) indexing").unwrap();
    for i in 0usize..s.len() {
        hprintln!("{},", bs[i]).unwrap();
    }

    hprintln!("").unwrap();

    //let a: [u8; 4] = [65u8; 4];
    let mut a = [0u8; 4];

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
//    answer: bare3
//            s = ABCD
//            bs = [65, 66, 67, 68]
//            iterate over slice
//            65,66,67,68,iterate iterate using (raw) indexing
//            65,
//            66,
//            67,
//            68,
//
//            a = AAAA
//
//    What is the type of `s`?
//
//    ** your answer here **
//    answer: &str
//
//    What is the type of `bs`?
//
//    ** your answer here **
//    answer: &[u8]
//
//    What is the type of `c`?
//
//    ** your answer here **
//    answer: &u8
//
//    What is the type of `a`?
//
//    ** your answer here **
//    answer: [u8, _]
//
//    What is the type of `i`?
//
//    ** your answer here **
//    answer: usize
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
// 4. Alter the program so that the data from `bs` is copied byte
//    by byte into `a` using a loop and raw indexing.
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
//
// 6. Optional
//    Rust is heavily influenced by functional languages.
//    Figure out how you can use an iterator to work over both
//    the `a` and `bs` to copy the content of `bs` to `a`.
//
//    You may use
//    - `iter` (to turn a slice into an iterator)
//    - `zip` (to merge two slices into an iterator)
//    - a for loop to assign the elements
//
//    Commit your solution (bare3_6)
//
// 7. Optional
//    Iter using `foreach` and a closure instead of the for loop.
//
//    Commit your solution (bare3_7)
//
// 8. Optional*
//    Now benchmark your different solutions using the cycle accurate
//    DWT based approach (in release mode).
//
//    Cycle count for `raw` indexing
//
//    ** your answer here **
//
//    Cycle count for the primitive slice approach.
//
//    ** your answer here **
//
//    Cycle count for the primitive slice approach.
//
//    ** your answer here **
//
//    Cycle count for the zip + for loop approach.
//
//    ** your answer here **
//
//    Cycle count for the zip + for_each approach.
//
//    What conclusions can you draw, does Rust give you zero-cost abstractions?
//
//    ** your answer here **
