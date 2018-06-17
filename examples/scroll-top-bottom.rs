#[cfg(feature = "default")]
extern crate sensehat_screen;

#[cfg(feature = "default")]
use sensehat_screen::{FontCollection, PixelColor, Screen, Scroll};

#[cfg(not(feature = "default"))]
fn main() {
    unimplemented!("This examples needs the 'default' features.");
}

#[cfg(feature = "default")]
fn main() {
    // Connect to our LED Matrix screen.
    let mut screen = Screen::open("/dev/fb1").unwrap();

    // Get the default `FontCollection`.
    let fonts = FontCollection::new();
    // Create a sanitized `FontString`.
    let sanitized = fonts.sanitize_str(" vvv123vvv ").unwrap();
    // Render the `FontString` as a vector of pixel frames, with
    // a stroke color of YELLOW and a BLACK background.
    let pixel_frames = sanitized.pixel_frames(PixelColor::YELLOW, PixelColor::BLACK);

    // Create a `Scroll` from the pixel frame vector.
    let scroll = Scroll::new(&pixel_frames);

    // Consume the `FrameSequence` returned by the `top_to_bottom` method.
    scroll.top_to_bottom().for_each(|frame| {
        println!("Now printing:");
        println!("{:?}", frame);
        screen.write_frame(&frame.frame_line());
        ::std::thread::sleep(::std::time::Duration::from_millis(250));
    });
}
