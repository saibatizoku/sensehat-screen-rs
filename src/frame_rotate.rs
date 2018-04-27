//! Frame rotation for the LED Matrix screen
use super::PixelFrame;
use super::PixelColor;

/// A counter-clockwise angle, multiple of `90°`, used to create rotated `PixelFrame`s.
pub enum Rotate {
    None,
    Ccw90,
    Ccw180,
    Ccw270,
}

/// Methods enabled by the `rotate` feature.
// TODO: Put this under `Rotate` trait & feature
// brute-force... TODO: optimize to in-place manipulation
impl PixelFrame {
    /// Create a new `PixelFrame` that is rotated by a multiple of `90°`, counter-clockwise.
    pub fn rotate(&self, rotate: Rotate) -> Self {
        match rotate {
            Rotate::None => self.clone(),
            Rotate::Ccw90 => self.rotate_left(),
            Rotate::Ccw180 => self.rotate_180(),
            Rotate::Ccw270 => self.rotate_right(),
        }
    }

    // Rotate the display to the left by 90 degrees. Creates a new `PixelFrame`.
    fn rotate_left(&self) -> Self {
        let transpose: Vec<PixelColor> = (0..8)
            .into_iter()
            .map(|col_idx| {
                let column: Vec<PixelColor> = self.0
                    .iter()
                    .enumerate()
                    .filter(|&(idx, _)| idx % 8 == col_idx)
                    .map(|(_, color)| *color)
                    .collect();
                column
            })
            .flat_map(|col| col)
            .collect();

        let flip_rows = transpose
            .chunks(8)
            .rev()
            .flat_map(|row| row.into_iter())
            .enumerate()
            .fold([PixelColor::default(); 64], |mut pxs, (idx, &px)| {
                pxs[idx] = px;
                pxs
            });

        PixelFrame(flip_rows)
    }

    // Rotate the display by 180 degrees. Creates a new `PixelFrame`.
    fn rotate_180(&self) -> Self {
        let flip_180 = self.0.iter().rev().enumerate().fold(
            [PixelColor::default(); 64],
            |mut pxs, (idx, &px)| {
                pxs[idx] = px;
                pxs
            },
        );
        PixelFrame(flip_180)
    }

    // Rotate the display to the right by 90 degrees. Creates a new `PixelFrame`.
    fn rotate_right(&self) -> Self {
        let rotated_tranpose = (0..8).into_iter().map(|col_idx| {
            let column: Vec<PixelColor> = self.0
                .iter()
                .enumerate()
                .filter(|&(idx, _)| idx % 8 == col_idx)
                .rev()
                .map(|(_, color)| *color)
                .collect();

            column
        });

        let flip_right = rotated_tranpose.flat_map(|col| col.into_iter())
            .enumerate()
            .fold([PixelColor::default(); 64], |mut pxs, (idx, px)| {
                pxs[idx] = px;
                pxs
            });

        PixelFrame(flip_right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DARK: PixelColor = PixelColor::BLACK;
    const BLUE: PixelColor = PixelColor::BLUE;

    const CHECKER_BASE: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, BLUE, DARK, BLUE, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, BLUE, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, DARK, //
        BLUE, DARK, DARK, DARK, DARK, DARK, BLUE, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
    ];

    const CHECKER_90_CW: [PixelColor; 64] = [
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, DARK, DARK, DARK, DARK, BLUE, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, //
        DARK, BLUE, BLUE, DARK, DARK, DARK, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, //
    ];

    const CHECKER_180: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, //
        DARK, BLUE, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, BLUE, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, BLUE, DARK, BLUE, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
    ];

    const CHECKER_90_CCW: [PixelColor; 64] = [
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, BLUE, BLUE, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, //
    ];

    #[test]
    fn pixel_frame_is_rotated_90_degrees_left() {
        let checker_base = PixelFrame(CHECKER_BASE);
        let checker_left = PixelFrame(CHECKER_90_CCW);
        assert_eq!(checker_base.rotate_left(), checker_left);
    }

    #[test]
    fn pixel_frame_is_rotated_by_180_degrees() {
        let checker_base = PixelFrame(CHECKER_BASE);
        let checker_180 = PixelFrame(CHECKER_180);
        assert_eq!(checker_base.rotate_180(), checker_180);
    }

    #[test]
    fn pixel_frame_is_rotated_by_180_degrees_by_two_90_deg_steps() {
        let checker_base = PixelFrame(CHECKER_BASE);
        let checker_180 = PixelFrame(CHECKER_180);
        assert_eq!(checker_base.rotate_left().rotate_left(), checker_180);
    }

    #[test]
    fn pixel_frame_is_rotated_90_degrees_right() {
        let checker_base = PixelFrame(CHECKER_BASE);
        let checker_right = PixelFrame(CHECKER_90_CW);
        assert_eq!(checker_base.rotate_right(), checker_right);
    }
}
