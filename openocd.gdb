target extended-remote :3333

set print asm-demangle on
monitor arm semihosting enable

monitor tpiu config internal /tmp/itm.fifo uart off 16000000 2000000

# enable itm ports
monitor itm port 0 on 
monitor itm port 1 on
monitor itm port 2 on

# *try* to stop at the user entry point (it might be gone due to inlining)
break main

# detect unhandled exceptions, hard faults and panics
break HardFault
break core::panicking::panic_fmt

# un-comment to check that flashing was successful
# compare-sections

monitor reset init
load
stepi