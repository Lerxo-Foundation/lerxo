/// Information provided by the Lerxo bootloader to the kernel.
#[repr(C)]
pub struct BootInfo {
    /// Physical address of the UEFI framebuffer.
    pub framebuffer_address: usize,

    /// Width of the framebuffer in pixels.
    pub framebuffer_width: usize,

    /// Height of the framebuffer in pixels.
    pub framebuffer_height: usize,

    /// Number of bytes between the start of two framebuffer rows.
    pub framebuffer_stride: usize,
}
