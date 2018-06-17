//! Scrolling for pixel frames on the LED Matrix.
use super::{Clip, Offset, PixelFrame};
use std::ops::Index;

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
pub struct FrameSequence {
    clips: Vec<Clip>,
    direction: FrameDirection,
    position: usize,
}

impl FrameSequence {
    /// Create a new `FrameSequence` from a reference to a `Scroll` and a `FrameDirection`.
    fn new(scroll: &Scroll, direction: FrameDirection) -> Self {
        let position = 0usize;
        let clips = scroll.clips();
        FrameSequence { clips,
                        direction,
                        position, }
    }

    pub fn positions(&self) -> usize {
        self.clips.len() * 8
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
    fn scroll_implements_index_trait_with_pixel_frame_output() {
        let scroll = Scroll::new(SCROLL_ONE);
        assert_eq!(scroll[0], BLK);
        assert_eq!(scroll[1], RED);
    }

    #[test]
    #[should_panic]
    fn scroll_implements_index_trait_with_pixel_frame_output_panics_when_out_of_bounds() {
        let scroll = Scroll::new(SCROLL_ONE);
        assert_eq!(scroll[4], BLK);
    }

    #[test]
    fn frame_sequence_positions_method_returns_calculated_positions_if_scroll_has_many_items() {
        let scroll =
            Scroll::new(&font_pixel_frames("bas  bas  ", PixelColor::YELLOW, PixelColor::BLACK));
        let sequence = scroll.left_to_right();
        assert_eq!(sequence.positions(), 72);
    }

    #[test]
    fn frame_sequence_iterator_count_equals_number_of_positions_plus_one() {
        let scroll = Scroll::new(&font_pixel_frames("bas", PixelColor::YELLOW, PixelColor::BLACK));
        let seq = scroll.left_to_right();
        let positions_plus_one = seq.positions() + 1;
        assert_eq!(seq.count(), positions_plus_one);
    }

    #[test]
    fn frame_sequence_implements_iterator_of_pixel_frames_left_to_right() {
        let scroll = Scroll::new(&font_pixel_frames("bás", PixelColor::YELLOW, PixelColor::BLACK));

        let mut seq = scroll.left_to_right();
        let first_frame = seq.nth(0).unwrap();
        assert_eq!(first_frame, scroll[0]);

        let mut seq = scroll.left_to_right();
        let nth_frame = seq.nth(1).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::right(1)));

        let mut seq = scroll.left_to_right();
        let nth_frame = seq.nth(2).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::right(2)));

        let mut seq = scroll.left_to_right();
        let nth_frame = seq.nth(3).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::right(3)));

        let mut seq = scroll.left_to_right();
        let nth_frame = seq.nth(4).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::right(4)));

        let mut seq = scroll.left_to_right();
        let nth_frame = seq.nth(5).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::right(5)));

        let mut seq = scroll.left_to_right();
        let nth_frame = seq.nth(6).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::right(6)));

        let mut seq = scroll.left_to_right();
        let nth_frame = seq.nth(7).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::right(7)));

        let mut seq = scroll.left_to_right();
        let eighth_frame = seq.nth(8).unwrap();
        assert_eq!(eighth_frame, scroll[1]);

        let mut seq = scroll.left_to_right();
        let nth_frame = seq.nth(9).unwrap();
        assert_eq!(nth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::right(1)));

        let mut seq = scroll.left_to_right();
        let nth_frame = seq.nth(10).unwrap();
        assert_eq!(nth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::right(2)));

        let mut seq = scroll.left_to_right();
        let nth_frame = seq.nth(11).unwrap();
        assert_eq!(nth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::right(3)));

        let mut seq = scroll.left_to_right();
        let nth_frame = seq.nth(12).unwrap();
        assert_eq!(nth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::right(4)));

        let mut seq = scroll.left_to_right();
        let twelfth_frame = seq.nth(13).unwrap();
        assert_eq!(twelfth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::right(5)));

        let mut seq = scroll.left_to_right();
        let nth_frame = seq.nth(14).unwrap();
        assert_eq!(nth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::right(6)));

        let mut seq = scroll.left_to_right();
        let nth_frame = seq.nth(15).unwrap();
        assert_eq!(nth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::right(7)));

        let mut seq = scroll.left_to_right();
        let last_frame = seq.nth(16).unwrap();
        assert_eq!(last_frame, scroll[2]);
    }

    #[test]
    fn frame_sequence_implements_iterator_of_pixel_frames_right_to_left() {
        let scroll = Scroll::new(&font_pixel_frames("áàä", PixelColor::WHITE, PixelColor::BLUE));

        let mut seq = scroll.right_to_left();
        let first_frame = seq.nth(0).unwrap();
        assert_eq!(first_frame, scroll[0]);

        let mut seq = scroll.right_to_left();
        let nth_frame = seq.nth(1).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::left(1)));

        let mut seq = scroll.right_to_left();
        let nth_frame = seq.nth(2).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::left(2)));

        let mut seq = scroll.right_to_left();
        let nth_frame = seq.nth(3).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::left(3)));

        let mut seq = scroll.right_to_left();
        let nth_frame = seq.nth(4).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::left(4)));

        let mut seq = scroll.right_to_left();
        let nth_frame = seq.nth(5).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::left(5)));

        let mut seq = scroll.right_to_left();
        let nth_frame = seq.nth(6).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::left(6)));

        let mut seq = scroll.right_to_left();
        let nth_frame = seq.nth(7).unwrap();
        assert_eq!(nth_frame, scroll[0].build_clip(&scroll[1]).offset(Offset::left(7)));

        let mut seq = scroll.right_to_left();
        let eighth_frame = seq.nth(8).unwrap();
        assert_eq!(eighth_frame, scroll[1]);

        let mut seq = scroll.right_to_left();
        let nth_frame = seq.nth(9).unwrap();
        assert_eq!(nth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::left(1)));

        let mut seq = scroll.right_to_left();
        let nth_frame = seq.nth(10).unwrap();
        assert_eq!(nth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::left(2)));

        let mut seq = scroll.right_to_left();
        let nth_frame = seq.nth(11).unwrap();
        assert_eq!(nth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::left(3)));

        let mut seq = scroll.right_to_left();
        let nth_frame = seq.nth(12).unwrap();
        assert_eq!(nth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::left(4)));

        let mut seq = scroll.right_to_left();
        let twelfth_frame = seq.nth(13).unwrap();
        assert_eq!(twelfth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::left(5)));

        let mut seq = scroll.right_to_left();
        let nth_frame = seq.nth(14).unwrap();
        assert_eq!(nth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::left(6)));

        let mut seq = scroll.right_to_left();
        let nth_frame = seq.nth(15).unwrap();
        assert_eq!(nth_frame, scroll[1].build_clip(&scroll[2]).offset(Offset::left(7)));

        let mut seq = scroll.right_to_left();
        let last_frame = seq.nth(16).unwrap();
        assert_eq!(last_frame, scroll[2]);
    }
}
