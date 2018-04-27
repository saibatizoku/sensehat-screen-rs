//! Framebuffer support for the Sense HAT LED Matrix.
use super::FrameLine;
use framebuffer::{Framebuffer, FramebufferError};

/// Framebuffered 8x8 LED screen.
#[derive(Debug)]
pub struct Screen {
    framebuffer: Framebuffer,
}

#[cfg(feature = "linux-framebuffer")]
impl Screen {
    /// Open the framebuffer to the screen at the given file-system path.
    pub fn open(path: &str) -> Result<Self, FramebufferError> {
        let framebuffer = Framebuffer::new(path)?;
        Ok(Screen { framebuffer })
    }

    /// Write the contents of a `FrameLine` into the framebuffer. This will
    /// render the frameline on the screen.
    pub fn write_frame(&mut self, frame: &FrameLine) {
        self.framebuffer.write_frame(&frame.as_bytes());
    }
}
