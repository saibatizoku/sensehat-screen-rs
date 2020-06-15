#[cfg(feature = "default")]
extern crate sensehat_screen;

#[cfg(feature = "default")]
use sensehat_screen::Offset;
#[cfg(feature = "default")]
use sensehat_screen::{font_to_pixel_frame, PixelColor, PixelFrame, Screen, FONT_COLLECTION};

#[cfg(not(feature = "default"))]
fn main() {
    unimplemented!("This examples needs the 'default' features.");
}

#[cfg(feature = "default")]
fn main() {
    let mut screen = Screen::open("/dev/fb1").unwrap();

    let letters = "a e i o u ";
    let letter_color = PixelColor::YELLOW.dim(0.5);

    let frames = letters
        .chars()
        .map(|sym| {
            let font = FONT_COLLECTION.get(sym).unwrap();
            font_to_pixel_frame(&font.byte_array(), letter_color)
        })
        .collect::<Vec<PixelFrame>>();

    // create a sequence of clips that will scroll each character-whitespace pair
    // from appearing to move from right to left.
    let frame_reel: Vec<PixelFrame> = frames.chunks(2).fold(Vec::new(), |mut v, chunk| match chunk
        .len()
    {
        2 => {
            let clip = chunk[0].build_clip(&chunk[1]);
            for i in 0..=8 {
                v.push(clip.offset(Offset::left(i)));
            }
            v
        }
        _ => panic!("this frame reel will only display &str of even length (divisible by 2)"),
    });

    for frame in &frame_reel {
        screen.write_frame(&frame.frame_line());
        ::std::thread::sleep(::std::time::Duration::from_millis(750));
    }
}
