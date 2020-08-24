# x96check

Check if the executable file is 32-bit or 64-bit on windows.

## Installing

``` shellsession
cargo install x96check
```

## Usage

``` shellsession
> x96check.exe -h
x96check 0.1.0
Check if the executable file is 32-bit or 64-bit on windows

USAGE:
    x96check.exe <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <file>    Full path executable file
```

## Examples

``` shellsession
> x96check.exe error_show.exe
64bit
```
