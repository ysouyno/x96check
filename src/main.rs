use std::ptr;
use structopt::StructOpt;

#[cfg(windows)]
extern crate winapi;
use winapi::um::fileapi;
use winapi::um::handleapi;
use winapi::um::winnt;

/// Check if the executable file is 32-bit or 64-bit on windows.
#[derive(StructOpt, Debug)]
struct Opts {
    /// Full path executable file.
    file: String,
}

#[cfg(windows)]
// Get a win32 lpstr from a &str, converting u8 to u16 and appending '\0'
fn to_wstring(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;

    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

#[cfg(windows)]
fn x96check(file: &str) {
    unsafe {
        let h = fileapi::CreateFileW(
            to_wstring(file).as_ptr(),
            winnt::GENERIC_READ,
            winnt::FILE_SHARE_READ | winnt::FILE_SHARE_WRITE,
            ptr::null_mut(),
            fileapi::OPEN_EXISTING,
            winnt::FILE_ATTRIBUTE_NORMAL,
            ptr::null_mut(),
        );
        if h == handleapi::INVALID_HANDLE_VALUE {
            println!("Open \"{}\" failed", file);
            return;
        }

        let filesize = fileapi::GetFileSize(h, ptr::null_mut());
        if filesize == 0 {
            println!("\"{}\" filesize is 0", file);
            return;
        }
    };
}

fn main() {
    let opts = Opts::from_args();

    x96check(&opts.file);
}
