extern crate sensehat_screen;

use sensehat_screen::{font_to_frame, FontCollection, PixelColor, Screen};
use std::thread;
use std::time::Duration;

fn main() {
    let mut screen = Screen::open("/dev/fb1").unwrap();

    let fonts = FontCollection::new();
    let white_50_pct = PixelColor::new(127, 127, 127);

    let letter = "Dear Toño, I am well. Thank you. Bye. - ゆにち";

    let screen_text = fonts.sanitize_str(letter).unwrap();

    for unicode in screen_text.to_slice() {
        if let Some(symbol) = fonts.get(*unicode) {
            let frame = font_to_frame(symbol, white_50_pct);
            screen.write_frame(&frame);
        }
        thread::sleep(Duration::from_millis(1_300));
    }
}
