use opencv::Error;
use thiserror::Error;
use crate::screenshot_error::ScreenshotError;

#[derive(Error, Debug)]
pub enum ImageMatchError {
    #[error(transparent)]
    Screenshot(#[from]ScreenshotError),
    #[error(transparent)]
    OpenCV(#[from]Error),
    #[error("无法读取图像: {0}")]
    CanNotReadImage(String),
}