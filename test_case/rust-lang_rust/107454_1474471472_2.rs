rust
#![no_std] // Dont use the Rust standard library
#![no_main] // Dont use rust entry points
#![feature(custom_test_frameworks)] // Lines 3, 4, 5: For testing purposes
#![test_runner(clarissa::test_runner)]
#![reexport_test_harness_main = "run_tests"]

use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// Assigning values to the enums to be used in the ColorCode constructor
#[allow(dead_code)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(u8)] // Displays memory as u8
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct ColorCode(u8);
impl ColorCode {
    pub fn new(background: Color, foreground: Color) -> ColorCode {
        // Bit manipulation operations
        // the first 4 bits are used to specify the background color of the character, the last 4 are used to specify the foreground color
        ColorCode((background as u8) << 4 | (foreground as u8)) // Somehow the enum values were not considered u8, a casting fixed this
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Copy, Clone)]
struct VgaChar {
    character: u8,
    color: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    // A matrix of characters representing the cga buffer in the gpu ram
    chars: [[Volatile<VgaChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column: usize, // Starting column on the buffer matrix, used to know where the next character should be written
    color: ColorCode,
    buffer: &'static mut Buffer, // Needs to be mutable because we need to write to it
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.new_line();
        } else {
            let row = BUFFER_HEIGHT - 1;
            let col = self.column;
            self.buffer.chars[row][col + 1].write(VgaChar {
                character: byte,
                color: self.color,
            });
            self.column += 1;
        }
    }
    fn new_line(&mut self) {
        // Shifts the characters written on the buffer up by one row and clears the bottom row
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row - 1][col].write(self.buffer.chars[row][col].read())
            }
        }
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[BUFFER_HEIGHT - 1][col].write(VgaChar {
                character: b' ',
                color: self.color,
            });
        }
        self.column = 0;
    }
    pub fn change_color(&mut self, color: ColorCode) {
        self.color = color
    }
}

impl Write for Writer {
    // Trait that allows formatting arguments
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(()) // We return Ok(()) because with our buffer system we can be sure that we are only performing safe writes to memory
    }
}

// We need a static instance of Writer that is also mutable
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column: 0,
        color: ColorCode::new(Color::Black, Color::White),
        // Make a mutable pointer to the vga buffer, dereference it, and borrow it as a mutable Buffer
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)}
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print_color {
    ($color: expr, $($arg:tt)*) => ($crate::color_macros::_print_color($color, format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println_color {
    () => ($crate::print!("\n"));
    ($color: expr, $($arg:tt)*) => ($crate::print_color!($color, "{}\n", format_args!($($arg)*)));
}
#[doc(hidden)]
pub fn _print_color(color: ColorCode, args: fmt::Arguments) {
    WRITER.lock().change_color(color);
    WRITER.lock().write_fmt(args).unwrap();
    WRITER.lock().change_color(ColorCode::new(Black, White));
}
#[macro_export]
macro_rules! blackln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Black), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! blueln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Blue), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! greenln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Green), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! cyanln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Cyan), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! redln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Red), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! magentaln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Magenta), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! brownln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Brown), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! lightgrayln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightGray), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! darkgrayln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, DarkGray), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! lightblueln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightBlue), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! lightgreenln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightGreen), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! lightcyanln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightCyan), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! lightredln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightRed), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! pinkln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Pink), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! yellowln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Yellow), "{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! whiteln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, White), "{}\n", format_args!($($arg)*)));
}

const VER: &str = "0.2.0"; // y.x.z = Section Z from chapter X, if Y is 0 the guide isn't finished, if its 1 or above it is
const NAME: &str = "try to remove old commits without breaking\nEVERYTHINNG CHALLENGE [IMPOSSOBLE 0.000000001% ONLY]";

// reminder of what mangling is: https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle] // Dont mangle the function name
pub extern "C" fn _start() -> ! {
    init();
    loop {}
}

// Panic function
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // Modern registry for kernel panics 😌
    lightredln!("Bruh moment! {}", _info);
    loop {}
}

#[test_case]
fn function27() {
    return ();
}
fn init() {
    redln!("Clarissa");
    darkgrayln!("        \\\\");
    lightgrayln!("          Ver {} - {}", VER, NAME);

    #[test_case]
    fn function27() {
        return ();
    }

    #[cfg(test)]
    println_serial!("\n          [main.rs]");
    #[cfg(test)]
    run_tests();
}

// Plans: Make tetris IN the operating system insanity
