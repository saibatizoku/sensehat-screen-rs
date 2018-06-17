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
    ///
    /// # Panics
    /// The scroll needs at least 2 PixelFrames to be created.
    pub fn new(frames: &[PixelFrame]) -> Self {
        assert!(frames.len() > 1);
        Scroll(frames.to_vec())
    }

    pub fn frames(&self) -> &[PixelFrame] {
        self.0.as_slice()
    }

    pub fn clips(&self) -> Vec<Clip> {
        let mut iter = self.0.iter().peekable();
        let mut clips = Vec::new();
        let mut base_frame = iter.next().unwrap();
        loop {
            match iter.next() {
                Some(next) => {
                    clips.push(base_frame.build_clip(next));
                    base_frame = next;
                }
                None => break,
            }
        }
        clips
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    pub fn len(&self) -> usize {
        self.0.len()
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

impl Index<usize> for Scroll {
    type Output = PixelFrame;

    fn index(&self, index: usize) -> &PixelFrame {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::super::{fonts::FontCollection, PixelColor};
    use super::*;

    const BLK: PixelFrame = PixelFrame::BLACK;
    const RED: PixelFrame = PixelFrame::RED;
    const YLW: PixelFrame = PixelFrame::YELLOW;

    const SCROLL_ONE: &[PixelFrame] = &[BLK, RED];
    const SCROLL_TWO: &[PixelFrame] = &[BLK, RED, YLW];

    // Helper function to generate a PixelFrame out of a utf16-encoded symbol,
    // a stroke color, and a background color.
    fn font_pixel_frames(s: &str, stroke: PixelColor, background: PixelColor) -> Vec<PixelFrame> {
        let fonts = FontCollection::new();
        let fstring = fonts.sanitize_str(s).unwrap();
        fstring.pixel_frames(stroke, background)
    }

    #[test]
    #[should_panic]
    fn scroll_is_created_from_empty_slice_of_pixel_frames_will_panic() {
        let _ = Scroll::new(&[]);
    }

    #[test]
    #[should_panic]
    fn scroll_is_created_from_slice_of_1_pixel_frame_will_panic() {
        let _ = Scroll::new(&[PixelFrame::BLUE]);
    }

    #[test]
    fn scroll_is_created_from_slice_of_at_least_2_pixel_frames() {
        let scroll = Scroll::new(SCROLL_ONE);
        assert_eq!(scroll, Scroll(SCROLL_ONE.to_vec()));
    }

    #[test]
    fn scroll_has_clips_method_returns_slice_of_clips() {
        let scroll = Scroll::new(SCROLL_ONE);
        let expected_clips = vec![BLK.build_clip(&RED)];
        assert_eq!(scroll.clips(), expected_clips);

        let scroll = Scroll::new(SCROLL_TWO);
        let expected_clips = vec![BLK.build_clip(&RED), RED.build_clip(&YLW)];
        assert_eq!(scroll.clips(), expected_clips);
    }

    #[test]
    fn scroll_has_frames_method_returns_slice_of_pixel_frames() {
        let scroll = Scroll::new(SCROLL_ONE);
        assert_eq!(scroll.frames(), SCROLL_ONE);
    }

    #[test]
    fn scroll_has_reverse_method_returns_slice_of_pixel_frames() {
        let mut scroll = Scroll::new(SCROLL_ONE);
        scroll.reverse();
        assert_eq!(scroll.frames(), &[RED, BLK]);
    }

    #[test]
    fn scroll_has_len_method_returns_the_number_of_pixel_frames() {
        let scroll = Scroll::new(&font_pixel_frames("áàäeéìiöòó",
                                                    PixelColor::WHITE,
                                                    PixelColor::BLUE));
        assert_eq!(scroll.len(), 10);
    }

    #[test]
    fn scrolls_create_right_to_left_frame_sequence() {
        let scroll = Scroll::new(SCROLL_ONE);
        let sequence = scroll.right_to_left();
        assert_eq!(sequence,
                   FrameSequence { clips: vec![BLK.build_clip(&RED)],
                                   direction: FrameDirection::RightToLeft,
                                   position: 0, });
    }

    #[test]
    fn scrolls_create_left_to_right_frame_sequence() {
        let scroll = Scroll::new(SCROLL_ONE);
        let sequence = scroll.left_to_right();
        assert_eq!(sequence,
                   FrameSequence { clips: vec![BLK.build_clip(&RED)],
                                   direction: FrameDirection::LeftToRight,
                                   position: 0, });
    }

    #[test]
    fn scrolls_create_top_to_bottom_frame_sequence() {
        let scroll = Scroll::new(SCROLL_ONE);
        let sequence = scroll.top_to_bottom();
        assert_eq!(sequence,
                   FrameSequence { clips: vec![BLK.build_clip(&RED)],
                                   direction: FrameDirection::TopToBottom,
                                   position: 0, });
    }

    #[test]
    fn scrolls_create_bottom_to_top_frame_sequence() {
        let scroll = Scroll::new(SCROLL_ONE);
        let sequence = scroll.bottom_to_top();
        assert_eq!(sequence,
                   FrameSequence { clips: vec![BLK.build_clip(&RED)],
                                   direction: FrameDirection::BottomToTop,
                                   position: 0, });
    }

    #[test]
    fn frame_sequence_positions_method_returns_calculated_positions_if_scroll_has_many_items() {
        let scroll =
            Scroll::new(&font_pixel_frames("bas  bas  ", PixelColor::YELLOW, PixelColor::BLACK));
        let sequence = scroll.left_to_right();
        assert_eq!(sequence.positions(), 72);
    }

    #[test]
    fn left_to_right_frame_sequence_is_a_collection_of_frame_clips() {
        let scroll = Scroll::new(&font_pixel_frames("bas", PixelColor::YELLOW, PixelColor::BLACK));
        let sequence = scroll.left_to_right();
        unimplemented!();
    }

    #[test]
    fn right_to_left_frame_sequence_is_a_collection_of_frame_clips() {
        let scroll = Scroll::new(&font_pixel_frames("áàä", PixelColor::WHITE, PixelColor::BLUE));
        let sequence = scroll.right_to_left();
        unimplemented!();
    }
}
