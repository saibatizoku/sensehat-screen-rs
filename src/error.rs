//! Errors for the SenseHat Screen.
#[cfg(feature = "linux-framebuffer")]
use framebuffer::FramebufferError;
#[cfg(feature = "fonts")]
use std::string::FromUtf16Error;

/// Errors that this crate can return
#[derive(Debug)]
pub enum ScreenError {
    #[cfg(feature = "linux-framebuffer")]
    Framebuffer(FramebufferError),
    #[cfg(feature = "fonts")]
    Unicode(FromUtf16Error),
}

#[cfg(feature = "linux-framebuffer")]
impl From<FramebufferError> for ScreenError {
    fn from(err: FramebufferError) -> ScreenError {
        ScreenError::Framebuffer(err)
    }
}

#[cfg(feature = "fonts")]
impl From<FromUtf16Error> for ScreenError {
    fn from(err: FromUtf16Error) -> ScreenError {
        ScreenError::Unicode(err)
    }
}
