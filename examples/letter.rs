extern crate sensehat_screen;

#[cfg(feature = "default")]
use sensehat_screen::{font_to_frame, PixelColor, Screen, FONT_COLLECTION};
#[cfg(feature = "default")]
use std::thread;
#[cfg(feature = "default")]
use std::time::Duration;

#[cfg(not(feature = "default"))]
fn main() {
    unimplemented!("This examples needs the 'default' features.");
}

#[cfg(feature = "default")]
fn main() {
    let mut screen = Screen::open("/dev/fb1").unwrap();

    let white_50_pct = PixelColor::WHITE.dim(0.5);

    let letter = "Dear Toño, I am well. Thank you. Bye. - ゆにち";

    let screen_text = FONT_COLLECTION.sanitize_str(letter).unwrap();

    for unicode in screen_text.chars() {
        if let Some(symbol) = FONT_COLLECTION.get(unicode) {
            let frame = font_to_frame(symbol.byte_array(), white_50_pct);
            screen.write_frame(&frame);
        }
        thread::sleep(Duration::from_millis(800));
    }
}
