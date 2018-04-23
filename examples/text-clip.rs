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

    let letters = "aeiou";
    let letter_color = PixelColor::YELLOW.dim(0.5);

    let frames = letters
        .encode_utf16()
        .map(|sym| {
            let font = fonts.get(sym).unwrap();
            font_to_pixel_frame(font, letter_color)
        })
        .collect::<Vec<PixelFrame>>();

    let frame_reel: Vec<PixelFrame> = frames.chunks(2).fold(Vec::new(), |mut v, chunk| {
        match chunk.len() {
            2 => {
                let clip = chunk[0].clip(&chunk[1]);
                for i in 0..=8 {
                    v.push(&clip.offset(Offset::left(i)));
                }
            }
            1 => {
            }
            0 => panic!("empty frame reel will display nothing"),
            _ => unreachable!("something strange is happening"),
    })
    if let Some(chunk) = iter.next() {
        render_chunk(&mut screen, chunk);
    }
}

fn render_chunk(screen: &mut Screen, chunk: &[PixelFrame]) {
    let clip = chunk[0].clip(&chunk[1]);
    for i in 0..=8 {
        screen.write_frame(&clip.offset(Offset::left(i)).frame_line());
        ::std::thread::sleep(::std::time::Duration::from_millis(500));
    }
    ::std::thread::sleep(::std::time::Duration::from_millis(500));
}
