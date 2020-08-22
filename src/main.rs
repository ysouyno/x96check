use structopt::StructOpt;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::slice;

#[cfg(windows)]
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

#[cfg(windows)]
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

#[cfg(windows)]
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

fn transmute_slice_u8_to_struct1(buf: &[u8]) -> ImageDosHeader {
    let p: *const [u8; std::mem::size_of::<ImageDosHeader>()] =
        buf.as_ptr() as *const [u8; std::mem::size_of::<ImageDosHeader>()];
    unsafe { std::mem::transmute(*p) }
}

fn transmute_slice_u8_to_struct2(buf: &[u8]) -> ImageNtHeaders {
    let p: *const [u8; std::mem::size_of::<ImageNtHeaders>()] =
        buf.as_ptr() as *const [u8; std::mem::size_of::<ImageNtHeaders>()];
    unsafe { std::mem::transmute(*p) }
}

fn read_structs<T, P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    let size = ::std::mem::size_of::<T>();
    let mut reader = BufReader::new(File::open(path)?);
    unsafe {
        let mut header: ImageDosHeader = std::mem::zeroed();
        let buffer = slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, size);
        reader.read_exact(buffer)?;
        let config = transmute_slice_u8_to_struct1(buffer);
        println!("{}", config.e_magic);

        reader.seek(std::io::SeekFrom::Start(header.e_lfanew as u64))?;

        let mut header: ImageFileHeader = std::mem::zeroed();
        let buffer = slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, size);
        reader.read_exact(buffer)?;
        let config = transmute_slice_u8_to_struct2(buffer);
        println!("{}", config.file_header.machine);
        if config.file_header.machine == 0x14c {
            println!("32bit");
        } else if config.file_header.machine == 0x8664 {
            println!("64bit");
        } else {
            println!("Unknown");
        }
    }
    Ok(())
}

#[cfg(windows)]
fn x96check(file: &str) {
    read_structs::<ImageDosHeader, _>(file).unwrap();
}

fn main() {
    let opts = Opts::from_args();

    x96check(&opts.file);
}
