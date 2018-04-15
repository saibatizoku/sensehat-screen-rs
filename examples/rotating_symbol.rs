#[cfg(feature = "default")]
extern crate sensehat_screen;

#[cfg(feature = "default")]
use sensehat_screen::{PixelColor, PixelFrame, Screen};

#[cfg(feature = "default")]
const DARK: PixelColor = PixelColor::BLACK;
#[cfg(feature = "default")]
const BLUE: PixelColor = PixelColor::BLUE;

#[cfg(feature = "default")]
const SYMBOL: [PixelColor; 64] = [
    DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
    DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
    DARK, DARK, BLUE, DARK, BLUE, DARK, DARK, DARK, //
    DARK, DARK, DARK, DARK, BLUE, DARK, DARK, DARK, //
    DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
    DARK, DARK, DARK, DARK, DARK, DARK, BLUE, DARK, //
    BLUE, DARK, DARK, DARK, DARK, DARK, BLUE, DARK, //
    BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
];

#[cfg(not(feature = "default"))]
fn main() {
    unimplemented!("This examples needs the 'default' features.");
}
#[cfg(feature = "default")]
fn main() {
    let mut screen = Screen::open("/dev/fb1").unwrap();
    let symbol = PixelFrame::new(&SYMBOL);
    let symbol_90 = symbol.clone().rotate_left();
    let symbol_180 = symbol.clone().rotate_left();
    let symbol_270 = symbol.clone().rotate_left();

    for _ in 0..10 {
        screen.write_frame(&symbol.frame_line());
        ::std::thread::sleep(::std::time::Duration::from_millis(250));
        screen.write_frame(&symbol_90.frame_line());
        ::std::thread::sleep(::std::time::Duration::from_millis(250));
        screen.write_frame(&symbol_180.frame_line());
        ::std::thread::sleep(::std::time::Duration::from_millis(250));
        screen.write_frame(&symbol_270.frame_line());
        ::std::thread::sleep(::std::time::Duration::from_millis(250));
    }
    screen.write_frame(&PixelFrame::new(&[PixelColor::BLACK; 64]).frame_line());
}
