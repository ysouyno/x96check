use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::slice;
use structopt::StructOpt;

const IMAGE_DOS_SIGNATURE: i16 = 0x5A4D; // MZ
const IMAGE_NT_SIGNATURE: i32 = 0x00004550; // PE00
const IMAGE_FILE_MACHINE_I386: u16 = 0x014C; // Intel 386.
const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664; // AMD64 (K8)

#[repr(C)]
struct ImageDosHeader {
    e_magic: i16,      // Magic number
    e_cblp: i16,       // Bytes on last page of file
    e_cp: i16,         // Pages in file
    e_crlc: i16,       // Relocations
    e_cparhdr: i16,    // Size of header in paragraphs
    e_minalloc: i16,   // Minimum extra paragraphs needed
    e_maxalloc: i16,   // Maximum extra paragraphs needed
    e_ss: i16,         // Initial (relative) SS value
    e_sp: i16,         // Initial SP value
    e_csum: i16,       // Checksum
    e_ip: i16,         // Initial IP value
    e_cs: i16,         // Initial (relative) CS value
    e_lfarlc: i16,     // File address of relocation table
    e_ovno: i16,       // Overlay number
    e_res: [i16; 4],   // Reserved words
    e_oemid: i16,      // OEM identifier (for e_oeminfo)
    e_oeminfo: i16,    // OEM information; e_oemid specific
    e_res2: [i16; 10], // Reserved words
    e_lfanew: i32,     // File address of new exe header
}

#[repr(C)]
struct ImageFileHeader {
    machine: u16,
    number_of_sections: i16,
    time_date_stamp: i32,
    pointer_to_symbol_table: i32,
    number_of_symbols: i32,
    size_of_optional_header: i16,
    characteristics: i16,
}

#[repr(C)]
struct ImageNtHeaders {
    signature: i32,
    file_header: ImageFileHeader,
}

/// Check if the executable file is 32-bit or 64-bit on windows.
#[derive(StructOpt, Debug)]
struct Opts {
    /// Full path executable file.
    file: String,
}

fn x96check(file: &str) -> io::Result<String> {
    let mut reader = BufReader::new(File::open(file)?);

    unsafe {
        let size = ::std::mem::size_of::<ImageDosHeader>();
        let mut header: ImageDosHeader = std::mem::zeroed();
        let header = slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, size);
        reader.read_exact(header)?;
        let header: ImageDosHeader = std::ptr::read(header.as_ptr() as *const _);
        if header.e_magic != IMAGE_DOS_SIGNATURE {
            return Ok(String::from("Not a valid DOS file."));
        }

        reader.seek(io::SeekFrom::Start(header.e_lfanew as u64))?;
        let size = ::std::mem::size_of::<ImageNtHeaders>();
        let mut header: ImageNtHeaders = std::mem::zeroed();
        let header = slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, size);
        reader.read_exact(header)?;
        let header: ImageNtHeaders = std::ptr::read(header.as_ptr() as *const _);
        if header.signature != IMAGE_NT_SIGNATURE {
            return Ok(String::from("Not a valid PE file."));
        }

        if header.file_header.machine == IMAGE_FILE_MACHINE_I386 {
            return Ok(String::from("32bit"));
        } else if header.file_header.machine == IMAGE_FILE_MACHINE_AMD64 {
            return Ok(String::from("64bit"));
        } else {
            return Ok(String::from("Unknown"));
        }
    }
}

fn main() {
    let opts = Opts::from_args();

    if cfg!(target_os = "windows") {
        let ret = x96check(&opts.file);
        match ret {
            Ok(ok) => println!("{}", ok),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("Only run on windows.");
    }
}
