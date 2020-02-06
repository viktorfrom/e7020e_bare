//! bare1.rs
//!
//! Inspecting the generated assembly
//!
//! What it covers
//! - tracing over semihosting and ITM
//! - assembly calls and inline assembly
//! - more on arithmetics

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;

#[entry]
#[inline(never)]
fn main() -> ! {
    // Prepend by `x` by _ to avoid warning (never used).
    // The compiler is smart enough to figure out that
    // `x` is not used in any menaningful way.

    let mut _x = 0;
    loop {
        _x += 1;
        cortex_m::asm::nop();
        cortex_m::asm::bkpt();
        _x -= 1;
    }
}

// 0. Setup
//    For this example we will use the `nightly` compiler
//    to get inline assembly.
//    (Inline assembly is currently not stabelized.)
//
//    > rustup override set nightly
//
//    In the `Corgo.toml` file, uncomment
//    # features = ["inline-asm"] # <- currently requires nightly compiler
//
//    You may need/want to install addititonal components also,
//    to that end look at the install section in the README.md.
//    If you change toolchain, exit and re-start `vscode`.
//
// 1. Build and run the application
//
//    > cargo build --example bare1
//    (or use the vscode build task)
//
//    Look at the `hello.rs` and `itm.rs` examples to setup the tracing.
//
//    When debugging the application it should get stuck in the
//    loop, (press pause/suspend to verify this).
//    what is the output in the ITM console
//
//    ** your answer here **
//
//    What is the output in the semihosting (openocd) console
//    ** your answer here **
//
//    Commit your answers (bare1_1)
//
// 2. Inspecting the generated assembly code
//    If in `vcsode` the gdb console in DEBUG CONSOLE
//
//    What is the output of:
//    (gdb) disassemble
//
//    ** your answer here **
//
//    Commit your answers (bare1_2)
//
// 3. Now remove the comment for `cortex_m::asm::nop()`.
//    Rebuild and debug, pause the program.
//
//    What is the output of:
//    (gdb) disassemble
//
//    ** your answer here **
//
//    Commit your answers (bare1_3)
//
// 4. Now remove the comment for `cortex_m::asm::bkpt()`
//    Rebuild and debug, let the program run until it halts.
//
//    What is the output of:
//    (gdb) disassemble
//
//    ** your answer here **
//
//    Commit your answers (bare1_4)
//
// 5. Release mode (optimized builds).
//    Rebuild `bare1.rs` in release (optimized mode).
//  
//    > cargo build --example bare1 --release
//    (or using the vscode build task)
//
//    Compare the generated assembly for the loop
//    between the dev (unoptimized) and release (optimized) build.
//
//    ** your answer here **
//
//    commit your answers (bare1_5)
//
//    Tips: The optimized build should have 3 instructions
//    while the debug (dev) build should have > 20 instructions
//    (both counting the inner loop only). The debug build
//    should have additional code that call panic if the additon
//    wraps (and in such case call panic).
//
//    Discussion:
//    In release (optimized) mode the addition is unchecked,
//    so there is a semantic difference here in between
//    the dev and release modes. This is motivited by:
//    1) efficiency, unchecked is faster
//    2) convenience, it would be inconvenient to explicitly use
//    wrapping arithmetics, and wrapping is what the programmer
//    typically would expect in any case. So the check
//    in dev/debug mode is just there for some extra safety
//    if your intention is NON-wrapping arithmetics.
//
// 6. *Optional
//    You can pass additional flags to the Rust `rustc` compiler.
//
//    `-Z force-overflow-checks=off`
//
//    Under this flag, code is never generated for oveflow checking.
//    You can enable this flag (uncomment the corresponding flag in
//    the `.cargo/config` file.)
//
//    What is now the disassembly of the loop (in debug mode):
//
//    ** your answer here **
//
//    commit your answers (bare1_6)
//
//    Now restore the `.cargo/config` to its original state.
//
// 7. *Optional
//    There is another way to conveniently use wrapping arithmetics
//    without passing flags to the compiler.
//
//    https://doc.rust-lang.org/std/num/struct.Wrapping.html
//
//    Rewrite the code using this approach.
//
//    What is now the disassembly of the code in dev mode?
//
//    ** your answer here **
//
//    What is now the disassembly of the code in release mode?
//
//    ** your answer here **
//
//    commit your answers (bare1_7)
//
//    Final discussion:
//
//    Embedded code typically is performance sensitve, hence
//    it is important to understand how code is generated
//    to achieve efficient implementations.
//
//    Moreover, arithmetics are key to processing of data,
//    so its important that we are in control over the
//    computations. E.g. comupting checksums, hashes, cryptos etc.
//    all require precise control over wrapping vs. overflow behaviour.
//
//    If you write a library depending on wrapping arithmetics
//    do NOT rely on a compiler flag. (The end user might compile
//    it without this flag enabled, and thus get erronous results.)
//
//    NOTICE:
//    ------
//    You are now on a `nightly` release of the compiler for good and bad.
//    You can chose to switch back to the stable channel. If so you must
//    restore the `Cargo.toml` (comment out the `features = ["inline-asm"]`)
//
//    Pros and cons of nightly:
//    + Acccess to new Rust features (such as inline assembly)
//    - No guarantee these features will work, they might change semantics,
//      or even be revoked.
//
//    The compiler itself is the same, the stable release is just a snapchot
//    of the nightly (released each 6 week). It is the latest nightly
//    that passed some additional regression test, not a different compiler.
//    And of course, the stable has the experimental features disabled.
//
//    So its up to you to decide if you want to use the stable or nightly.
