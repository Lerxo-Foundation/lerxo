use goblin::elf64::program_header::{ProgramHeader, PT_LOAD};

/// Represents a validated ELF64 kernel image.
pub struct KernelElf<'a> {
    data: &'a [u8],
    entry: u64,
}

impl<'a> KernelElf<'a> {
    /// Parses and validates a Lerxo kernel ELF image.
    pub fn parse(data: &'a [u8]) -> Result<Self, &'static str> {
        let elf = goblin::elf64::Elf::parse(data)
            .map_err(|_| "Invalid ELF image")?;

        if elf.header.e_type != goblin::elf64::header::ET_EXEC {
            return Err("Kernel is not an executable ELF");
        }

        if elf.header.e_machine != goblin::elf64::header::EM_X86_64 {
            return Err("Kernel is not x86-64");
        }

        if elf.program_headers.iter().all(|header| header.p_type != PT_LOAD) {
            return Err("Kernel contains no loadable segments");
        }

        Ok(Self {
            data,
            entry: elf.entry,
        })
    }

    /// Returns the kernel entry-point address.
    pub fn entry(&self) -> u64 {
        self.entry
    }

    /// Returns the raw kernel image.
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    /// Returns the kernel's loadable ELF segments.
    pub fn loadable_segments(&self) -> impl Iterator<Item = &ProgramHeader> {
        let elf = goblin::elf64::Elf::parse(self.data)
            .expect("kernel ELF was already validated");

        elf.program_headers
            .iter()
            .filter(|header| header.p_type == PT_LOAD)
    }
}
