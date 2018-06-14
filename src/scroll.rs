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

    #[test]
    fn scrolls_create_right_to_left_frame_sequence() {
        let scroll = Scroll::new(SCROLL_ONE);
        let sequence = scroll.right_to_left();
        assert_eq!(sequence, FrameSequence { scroll: &scroll, direction: FrameDirection::RightToLeft });
    }

    #[test]
    fn scrolls_create_left_to_right_frame_sequence() {
        let scroll = Scroll::new(SCROLL_ONE);
        let sequence = scroll.left_to_right();
        assert_eq!(sequence, FrameSequence { scroll: &scroll, direction: FrameDirection::LeftToRight });
    }

    #[test]
    fn scrolls_create_top_to_bottom_frame_sequence() {
        let scroll = Scroll::new(SCROLL_ONE);
        let sequence = scroll.top_to_bottom();
        assert_eq!(sequence, FrameSequence { scroll: &scroll, direction: FrameDirection::TopToBottom });
    }

    #[test]
    fn scrolls_create_bottom_to_top_frame_sequence() {
        let scroll = Scroll::new(SCROLL_ONE);
        let sequence = scroll.bottom_to_top();
        assert_eq!(sequence, FrameSequence { scroll: &scroll, direction: FrameDirection::BottomToTop });
    }
}
