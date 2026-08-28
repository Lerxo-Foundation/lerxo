#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Kernel panic handler.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Lerxo kernel entry point.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    lerxo_kernel::arch::x86_64::initialise();

    loop {}
}
