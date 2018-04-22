#[cfg(feature = "default")]
extern crate sensehat_screen;

#[cfg(feature = "default")]
use sensehat_screen::{font_to_pixel_frame, FontCollection, PixelColor, PixelFrame, Screen};
use sensehat_screen::frame::offset::Offset;

#[cfg(not(feature = "default"))]
fn main() {
    unimplemented!("This examples needs the 'default' features.");
}
#[cfg(feature = "default")]
fn main() {
    let mut screen = Screen::open("/dev/fb1").unwrap();
    let fonts = FontCollection::new();

    for &(sym, color) in &[('þ', PixelColor::CYAN), ('ß', PixelColor::WHITE.dim(0.5))] {
        let font = fonts.get(sym as u16).unwrap();
        let symbol = font_to_pixel_frame(font, color);

        // Starts with an empty screen, then the symbol slides from the left,
        // reaching the offset = 0 position, which renders the entire symbol on
        // the screen.
        for i in 0..=8 {
            screen.write_frame(&symbol.offset(Offset::left(8 - i)).frame_line());
            ::std::thread::sleep(::std::time::Duration::from_millis(500));
        }
        // Slides the displayed symbol to the right until it disappears.
        for i in 0..=8 {
            screen.write_frame(&symbol.offset(Offset::right(i)).frame_line());
            ::std::thread::sleep(::std::time::Duration::from_millis(500));
        }

        // Starts with an empty screen, then the symbol slides from the top,
        // reaching the offset = 0 position, which renders the entire symbol on
        // the screen.
        for i in 0..=8 {
            screen.write_frame(&symbol.offset(Offset::top(8 - i)).frame_line());
            ::std::thread::sleep(::std::time::Duration::from_millis(500));
        }
        // Slides the displayed symbol to the bottom until it disappears.
        for i in 0..=8 {
            screen.write_frame(&symbol.offset(Offset::bottom(i)).frame_line());
            ::std::thread::sleep(::std::time::Duration::from_millis(500));
        }
    }
    screen.write_frame(&PixelFrame::new(&[PixelColor::BLACK; 64]).frame_line());
}
