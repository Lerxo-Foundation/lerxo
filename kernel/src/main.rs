#![no_std]
#![no_main]

use core::panic::PanicInfo;
use lerxo_kernel::boot::BootInfo;

/// Kernel panic handler.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Lerxo kernel entry point.
#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    lerxo_kernel::arch::x86_64::initialise();

    let _ = boot_info;

    loop {}
}
