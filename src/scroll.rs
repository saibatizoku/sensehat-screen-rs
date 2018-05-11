//! Scrolling for pixel frames on the LED Matrix.
use super::PixelFrame;

/// A type representing a collection of `PixelFrame`s that may be scrolled.
#[derive(Debug, PartialEq)]
pub struct Scroll(Vec<PixelFrame>);

impl Scroll {
    /// Creates a new scroll from a slice of `PixelFrame`s.
    pub fn new(frames: &[PixelFrame]) -> Self {
        Scroll(frames.to_vec())
    }

    pub fn frames(&self) -> &[PixelFrame] {
        self.0.as_slice()
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCROLL_ONE: &[PixelFrame] = &[PixelFrame::BLACK, PixelFrame::RED];

    #[test]
    fn scrolls_are_created_from_slice_of_pixel_frames() {
        let scroll = Scroll::new(SCROLL_ONE);
        assert_eq!(scroll, Scroll(SCROLL_ONE.to_vec()));
    }


    #[test]
    fn scrolls_frames_method_returns_slice_of_pixel_frames() {
        let scroll = Scroll::new(SCROLL_ONE);
        assert_eq!(scroll.frames(), SCROLL_ONE);
    }

    #[test]
    fn scrolls_reverse_method_returns_slice_of_pixel_frames() {
        let mut scroll = Scroll::new(SCROLL_ONE);
        scroll.reverse();
        assert_eq!(scroll.frames(), &[PixelFrame::RED, PixelFrame::BLACK]);
    }
}
