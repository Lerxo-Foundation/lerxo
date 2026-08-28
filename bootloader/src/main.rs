#![no_main]
#![no_std]

use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, _system_table: SystemTable<Boot>) -> Status {
    loop {}
}
