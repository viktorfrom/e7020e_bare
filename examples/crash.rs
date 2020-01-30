//! Debugging a crash (exception)

#![no_main]
#![no_std]

extern crate panic_halt;
use core::ptr;

use cortex_m_rt::{entry, exception};

#[entry]
#[inline(never)]
fn main() -> ! {
    unsafe {
        // read an address outside of the RAM region to cause a HardFault exception
        ptr::read_volatile(0x2FFF_FFFF as *const u32);
    }

    loop {
        continue;
    }
}

#[exception]
#[inline(never)]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    // to inline never and read_volatile required to
    // avoid `ef` being optimized out in release mode
    unsafe {
        ptr::read_volatile(ef);
    }

    loop {
        continue;
    }
}

//
// Most crash conditions trigger a hard fault exception, whose handler is defined via
// ``` rust
// #[exception]
// fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
// ...
// ```
//
// `cortex-m-rt` generates a trampoline, that calls into your user defined `
// HardFault` handler. We look at the generated trampoline:
//
// ``` rust
// #[doc(hidden)]
// #[export_name = "HardFault"]
// #[link_section = ".HardFault.user"]
// pub unsafe extern "C" fn __cortex_m_rt_HardFault_trampoline(frame: &::cortex_m_rt::ExceptionFrame) {
//    __cortex_m_rt_HardFault(frame)
// }
// ```
//
// The `HardFault` handler has access to the exception `frame`, a
// snapshot of the CPU registers at the moment of the exception.
//
// To better see what is happening we make a `--release` build
// (It reduces the amount of redundant code.)
//
// ``` text
// $ cargo run --example crash --release
// ...
// Breakpoint 1, main () at examples/crash.rs:69
// (gdb) continue
// Breakpoint 3, HardFault (frame=0x20007fe0) at examples/crash.rs:82
// 82      #[exception]
// (gdb) p/x *frame
// $1 = cortex_m_rt::ExceptionFrame {r0: 0x2fffffff, r1: 0xf00000, r2: 0x0, r3: 0x0, r12: 0x0, lr: 0x8000405, pc: 0x800040a, xpsr: 0x61000000}
// (gdb) disassemble frame.pc
// Dump of assembler code for function crash::__cortex_m_rt_main:
//    0x08000406 <+0>:     mvn.w   r0, #3489660928 ; 0xd0000000
//    0x0800040a <+4>:     ldr     r0, [r0, #0]
//    0x0800040c <+6>:     b.n     0x800040c <crash::__cortex_m_rt_main+6>
// End of assembler dump.
// ```
//
// The program counter (frame.pc) contains the address of the instruction that caused the exception. In GDB one can
// disassemble the program around this address to observe the instruction that caused the
// exception. In our case its the `ldr r0, [r0, #0]` caused the exception. This instruction tried to load (read) a 32-bit word
// from the address stored in the register `r0`. Looking again at the contents of `ExceptionFrame`
// we find that `r0` contained the address `0x2FFF_FFFF` when this instruction was executed.
//
// Looking at the assembly `mvn.w   r0, #3489660928 ; 0xd0000000`.
// This is a *move* and *not*, so the resulting value here is actually
// 0x2fffffff. Why did it not do it straight up then as 0x2FFF_FFFF?
//
// Well a 32 bit constant cannot be stored in a 32 bit instruction.
// So under the hood it stores 0xd0, bit shifts it and bit wise inversion.
// This is the level of optimization Rust + LLVM is capable of.
//
// We can further backtrace the calls leading up to the fault.
// ``` text
// ((gdb) bt
// #0  HardFault (frame=0x20007fe0) at examples/crash.rs:79
// #1  <signal handler called>
// #2  core::ptr::read_volatile (src=0x2fffffff)
//     at /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/ptr/mod.rs:948
// #3  crash::__cortex_m_rt_main () at examples/crash.rs:71
// #4  0x08000404 in main () at examples/crash.rs:66
// ```
// Here we see that on frame #2 we are doing the read causing havoc.
