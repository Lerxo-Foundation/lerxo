use alloc::vec::Vec;
use uefi::boot;
use uefi::fs::FileSystem;

/// Path to the Lerxo kernel on the EFI System Partition.
const KERNEL_PATH: &str = "\\EFI\\LERXO\\LERXO.KRN";

/// Loads the Lerxo kernel from the EFI System Partition.
pub fn load_kernel() -> Result<Vec<u8>, uefi::fs::FileSystemError> {
    let handle = boot::image_handle();
    let filesystem = boot::get_image_file_system(handle)
        .map_err(|error| error)?;

    let mut filesystem = FileSystem::new(filesystem);

    filesystem.read(KERNEL_PATH)
}
