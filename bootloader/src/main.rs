#![no_main]
#![no_std]

extern crate alloc;

use uefi::prelude::*;

mod filesystem;

#[entry]
fn main(
    _image_handle: Handle,
    mut system_table: SystemTable<Boot>,
) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();

    log::info!("Welcome to the Lerxo Bootloader!");

    match filesystem::load_kernel() {
        Ok(kernel) => {
            log::info!("Loaded Lerxo kernel: {} bytes", kernel.len());
        }
        Err(error) => {
            log::error!("Failed to load Lerxo kernel: {:?}", error);
        }
    }

    loop {}
}
