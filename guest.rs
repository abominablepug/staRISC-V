#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
    ".section .text.boot",
    ".global _start",
    "_start:",
    "    li x5, 0x10000000", // Store UART in register x5
    "    li x6, 0x00",       // Store '0' in register x6 (index)
    "    li x7, 0x0a",       // Store '10' in register x7
    "    li x8, 0x00",       // Store '0' in register x8 (first Fibonacci number)
    "    li x9, 0x01",       // Store '1' in register x9 (second Fibonacci number)
    "fibonacci:",
    "    sw x8, 0(x5)",          // Print x10 to UART
    "    add x10, x8, x9",       // Calculate next Fibonacci number
    "    add x8, x0, x9",        // Update x8
    "    add x9, x0, x10",       // Update x9
    "    addi x6, x6, 1",        // Increment index
    "    bne x6, x7, fibonacci", // Loop until x7
    "1:  j 1b",                  // Halt CPU
);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
