use goblin::elf64::{
    header::{ET_EXEC, EM_X86_64},
    program_header::{ProgramHeader, PT_LOAD},
    Elf,
};

pub struct KernelElf<'a> {
    data: &'a [u8],
    entry: u64,
}

impl<'a> KernelElf<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, &'static str> {
        let elf = Elf::parse(data)
            .map_err(|_| "Invalid ELF image")?;

        if elf.header.e_type != ET_EXEC {
            return Err("Kernel is not an executable ELF");
        }

        if elf.header.e_machine != EM_X86_64 {
            return Err("Kernel is not an x86-64 ELF");
        }

        if !elf
            .program_headers
            .iter()
            .any(|header| header.p_type == PT_LOAD)
        {
            return Err("Kernel contains no loadable segments");
        }

        Ok(Self {
            data,
            entry: elf.entry,
        })
    }

    pub fn entry(&self) -> u64 {
        self.entry
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }

    pub fn loadable_segments(&self) -> impl Iterator<Item = &ProgramHeader> {
        let elf = Elf::parse(self.data)
            .expect("kernel ELF was already validated");

        elf.program_headers
            .iter()
            .filter(|header| header.p_type == PT_LOAD)
    }
}
