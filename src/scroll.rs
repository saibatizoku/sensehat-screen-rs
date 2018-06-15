//! Scrolling for pixel frames on the LED Matrix.
use super::PixelFrame;

/// A sequence of frames
#[derive(Debug, PartialEq)]
pub enum FrameDirection {
    RightToLeft,
    LeftToRight,
    BottomToTop,
    TopToBottom,
}

/// A sequence of frames to be scrolled on the LED Matrix.
#[derive(Debug, PartialEq)]
pub struct FrameSequence<'a> {
    scroll: &'a Scroll,
    direction: FrameDirection,
}

impl<'a> FrameSequence<'a> {
    /// Create a new `FrameSequence` from a reference to a `Scroll` and a `FrameDirection`.
    fn new(scroll: &'a Scroll, direction: FrameDirection) -> Self {
        FrameSequence { scroll, direction }
    }
}

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

    pub fn right_to_left(&self) -> FrameSequence {
        FrameSequence::new(self, FrameDirection::RightToLeft)
    }

    pub fn left_to_right(&self) -> FrameSequence {
        FrameSequence::new(self, FrameDirection::LeftToRight)
    }

    pub fn top_to_bottom(&self) -> FrameSequence {
        FrameSequence::new(self, FrameDirection::TopToBottom)
    }

    pub fn bottom_to_top(&self) -> FrameSequence {
        FrameSequence::new(self, FrameDirection::BottomToTop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{fonts::FontCollection, PixelColor};

    const SCROLL_ONE: &[PixelFrame] = &[PixelFrame::BLACK, PixelFrame::RED];

    // Helper function to generate a PixelFrame out of a utf16-encoded symbol,
    // a stroke color, and a background color.
    fn font_pixel_frames(s: &str, stroke: PixelColor, background: PixelColor) -> Vec<PixelFrame> {
        let fonts = FontCollection::new();
        let fstring = fonts.sanitize_str(s).unwrap();
        fstring.pixel_frames(stroke, background)
    }

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

    #[test]
    fn scrolls_create_right_to_left_frame_sequence() {
        let scroll = Scroll::new(SCROLL_ONE);
        let sequence = scroll.right_to_left();
        assert_eq!(sequence,
                   FrameSequence { scroll: &scroll,
                                   direction: FrameDirection::RightToLeft });
    }

    #[test]
    fn scrolls_create_left_to_right_frame_sequence() {
        let scroll = Scroll::new(SCROLL_ONE);
        let sequence = scroll.left_to_right();
        assert_eq!(sequence,
                   FrameSequence { scroll: &scroll,
                                   direction: FrameDirection::LeftToRight });
    }

    #[test]
    fn scrolls_create_top_to_bottom_frame_sequence() {
        let scroll = Scroll::new(SCROLL_ONE);
        let sequence = scroll.top_to_bottom();
        assert_eq!(sequence,
                   FrameSequence { scroll: &scroll,
                                   direction: FrameDirection::TopToBottom });
    }

    #[test]
    fn scrolls_create_bottom_to_top_frame_sequence() {
        let scroll = Scroll::new(SCROLL_ONE);
        let sequence = scroll.bottom_to_top();
        assert_eq!(sequence,
                   FrameSequence { scroll: &scroll,
                                   direction: FrameDirection::BottomToTop });
    }

    #[test]
    fn left_to_right_frame_sequence_is_a_collection_of_frame_clips() {
        let scroll = Scroll::new(&font_pixel_frames("basic latin", PixelColor::YELLOW, PixelColor::BLACK));
        let sequence = scroll.left_to_right();
        unimplemented!();
    }

    #[test]
    fn right_to_left_frame_sequence_is_a_collection_of_frame_clips() {
        let scroll = Scroll::new(&font_pixel_frames("¡extended latin!", PixelColor::WHITE, PixelColor::BLUE));
        let sequence = scroll.right_to_left();
        unimplemented!();
    }
}
