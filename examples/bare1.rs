//! bare1.rs
//!
//! Inspecting the generated assembly
//!
//! What it covers
//! - Rust panic tracing using ITM
//! - assembly calls and inline assembly
//! - more on arithmetics

#![no_main]
#![no_std]

use panic_itm as _;

use cortex_m_rt::entry;

#[entry]
#[inline(never)]
fn main() -> ! {
    let mut x = core::u32::MAX - 1;
    loop {
        cortex_m::asm::bkpt();
        x += 1;
        cortex_m::asm::bkpt();

        // prevent optimization by read-volatile (unsafe)
        unsafe {
            //core::ptr::read_volatile(&x);
        }
    }
}

// 0. Setup
//    For this example we will use the `nightly` compiler
//    to get inline assembly.
//    (Inline assembly is currently not stabilized.)
//
//    > rustup override set nightly
//
//    In the `Cargo.toml` file, uncomment
//    # features = ["inline-asm"] # <- currently requires nightly compiler
//
//    The first time you use the new toolchain you may need to install the target.
//    > rustup target add thumbv7em-none-eabihf
//
//    You may need/want to install additional components also.
//    To that end look at the install section in the README.md.
//    (If you change toolchain, you may need to exit and re-start `vscode`.)
//
// 1. Build and run the application
//
//    > cargo run --example bare1
//    (or use the `itm fifo (debug)` or the `itm internal (debug)` launch configuration.)
//
//    Make sure you have followed the instructions for fifo `ITM` tracing accordingly.
//
//    When debugging the application it should hit the `bkpt` instruction.
//    What happens when you continue (second iteration of the loop)?
//    (passing 3 breakpoints)
//
//    ** your answer here **
//    answer: the program saves the panic message into an itmdump file.
//            itmdump -f /tmp/itm.fifo
//
//    What is the `ITM` output.
//
//    ** your answer here **
//    answer: panicked at 'attempt to add with overflow', examples/bare1.rs:23:9
//
//    Commit your answer (bare1_1)
//
// 2. Inspecting the generated assembly code.
//    Close and re-start the debug session. Run till you hit the `bkpt` instruction.
//
//    Under DEBUG CONSOLE you find the `gdb` interface.
//
//    What is the output of:
//    > disassemble
//
//    ** your answer here **
//    answer: Dump of assembler code for function bare1::__cortex_m_rt_main:
//            0x0800040e <+0>:	push	{r7, lr}
//            0x08000410 <+2>:	mov	r7, sp
//            0x08000412 <+4>:	sub	sp, #8
//            0x08000414 <+6>:	mvn.w	r0, #1
//            0x08000418 <+10>:	str	r0, [sp, #4]
//         => 0x0800041a <+12>:	bkpt	0x0000
//            0x0800041c <+14>:	mov.w	r0, #4294967295	; 0xffffffff
//            0x08000420 <+18>:	add	r4, sp, #4
//            0x08000422 <+20>:	str	r0, [sp, #4]
//            0x08000424 <+22>:	bkpt	0x0000
//            0x08000426 <+24>:	mov	r0, r4
//            0x08000428 <+26>:	bl	0x8000400 <core::ptr::read_volatile>
//            0x0800042c <+30>:	bkpt	0x0000
//            0x0800042e <+32>:	ldr	r0, [sp, #4]
//            0x08000430 <+34>:	adds	r0, #1
//            0x08000432 <+36>:	bcc.n	0x8000422 <bare1::__cortex_m_rt_main+20>
//            0x08000434 <+38>:	movw	r0, #6352	; 0x18d0
//            0x08000438 <+42>:	movt	r0, #2048	; 0x800
//            0x0800043c <+46>:	movw	r2, #6324	; 0x18b4
//            0x08000440 <+50>:	movt	r2, #2048	; 0x800
//            0x08000444 <+54>:	movs	r1, #28
//            0x08000446 <+56>:	bl	0x8000550 <core::panicking::panic>
//            0x0800044a <+60>:	udf	#254	; 0xfe
//            End of assembler dump.
//
//    How many instructions are in between the two `bkpt` instructions in the loop.
//    Notice, the generated code may not be exactly what you expect :)
//
//    ** your answer here **
//    answer: 3
//
//    Which instruction stores the local variable on the stack.
//
//    ** your answer here **
//    answer: push
//
//    Commit your answers (bare1_2)
//
// 3. Release mode (optimized builds).
//    Rebuild `bare1.rs` in release (optimized mode).
//
//    > cargo build --example bare1 --release
//    (or using the vscode)
//
//    Compare the generated assembly for the loop
//    between the dev (un-optimized) and release (optimized) build.
//
//    What is the output of:
//    > disassemble
//
//    ** your answer here **
//    answer: Dump of assembler code for function bare1::__cortex_m_rt_main:
//            0x08000406 <+0>:	sub	sp, #4
//            0x08000408 <+2>:	mvn.w	r0, #1
//            0x0800040c <+6>:	str	r0, [sp, #0]
//            0x0800040e <+8>:	adds	r0, #1
//            0x08000410 <+10>:	bkpt	0x0000
//            0x08000412 <+12>:	str	r0, [sp, #0]
//            => 0x08000414 <+14>:	bkpt	0x0000
//            0x08000416 <+16>:	ldr	r0, [sp, #0]
//            0x08000418 <+18>:	b.n	0x800040e <bare1::__cortex_m_rt_main+8>
//            End of assembler dump.
//
//    How many instructions are in between the two `bkpt` instructions.
//    answer: 1
//
//    ** your answer here **
//    answer: 23
//
//    Where is the local variable stored?
//
//    ** your answer here **
//    answer: 0x08000412 <+12>:	str	r0, [sp, #0]
//
//    Is there now any reference to the panic handler?
//    If not, why is that the case?
//    answer: No, there is no reference.
//
//    commit your answers (bare1_3)
//
//    Discussion:
//    In release (optimized) mode the addition is unchecked,
//    so there is a semantic difference here in between
//    the dev and release modes. This is motivated by:
//    1) efficiency, unchecked is faster
//    2) convenience, it would be inconvenient to explicitly use
//    wrapping arithmetics, and wrapping is what the programmer
//    typically would expect in any case. So the check
//    in dev/debug mode is just there for some extra safety
//    if your intention is NON-wrapping arithmetics.
//
//    The debug build should have additional code that checks if the addition
//    wraps (and in such case call panic). In the case of the optimized
//    build there should be no reference to the panic handler in the generated
//    binary. Recovering from a panic is in general very hard. Typically
//    the best we can do is to stop and report the error (and maybe restart).
//
//    Later we will demonstrate how we can get guarantees of panic free execution.
//    This is very important to improve reliability.
//
// 4. Now comment out the `read_volatile`.
//
//    > cargo build --example bare1 --release
//    (or using the vscode)
//
//    Compare the generated assembly for the loop
//    between the dev (un-optimized) and release (optimized) build.
//
//    What is the output of:
//    > disassemble
//
//    ** your answer here **
//    answer: Dump of assembler code for function bare1::__cortex_m_rt_main:
//            => 0x0800040a <+0>:	bkpt	0x0000
//            0x0800040c <+2>:	bkpt	0x0000
//            0x0800040e <+4>:	b.n	0x800040a <bare1::__cortex_m_rt_main>
//            End of assembler dump.
//
//    How many instructions are in between the two `bkpt` instructions.
//
//    ** your answer here **
//    answer: 0
//
//    Where is the local variable stored?
//    What happened, and why is Rust + LLVM allowed to do that?
//
//    ** your answer here **
//    answer: Release mode applies the highest optimazation grade 
//            and is therefore allowed to remove certain instructions.
//
//    commit your answers (bare1_4)
//
//
// 5. *Optional
//    You can pass additional flags to the Rust `rustc` compiler.
//
//    `-Z force-overflow-checks=off`
//
//    Under this flag, code is never generated for overflow checking even in
//    non optimized (debug/dev) builds.
//    You can enable this flag in the `.cargo/config` file.
//  
//    What is now the disassembly of the loop (in debug/dev mode):
//
//    ** your answer here **
//
//    commit your answers (bare1_5)
//
//    Now restore the `.cargo/config` to its original state.
//
// 6. *Optional
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
//    commit your answers (bare1_6)
//
//    Final discussion:
//
//    Embedded code typically is performance sensitive, hence
//    it is important to understand how code is generated
//    to achieve efficient implementations.
//
//    Moreover, arithmetics are key to processing of data,
//    so its important that we are in control over the
//    computations. E.g. computing checksums, hashes, cryptos etc.
//    all require precise control over wrapping vs. overflow behavior.
//
//    If you write a library depending on wrapping arithmetics
//    do NOT rely on a compiler flag. (The end user might compile
//    it without this flag enabled, and thus get erroneous results.)
//
//    NOTICE:
//    ------
//    You are now on a `nightly` release of the compiler for good and bad.
//    You can chose to switch back to the stable channel. If so you must
//    restore the `Cargo.toml` (comment out the `features = ["inline-asm"]`)
//
//    Pros and cons of nightly:
//    + Access to new Rust features (such as inline assembly)
//    - No guarantee these features will work, they might change semantics,
//      or even be revoked.
//
//    The compiler itself is the same, the stable release is just a snapshot
//    of the nightly (released each 6 week). It is the latest nightly
//    that passed some additional regression test, not a different compiler.
//    And of course, the stable has the experimental features disabled.
//
//    So its up to you to decide if you want to use the stable or nightly.
