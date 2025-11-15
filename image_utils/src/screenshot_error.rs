use ndarray::ShapeError;
use thiserror::Error;
use xcap::XCapError;

#[derive(Debug, Error)]
pub enum ScreenshotError {
    #[error(transparent)]
    Capture(#[from]XCapError),
    #[error(transparent)]
    Shape(#[from] ShapeError),
    #[error("未找到任何可用监视器")]
    NoMonitorFound,
    #[error(transparent)]
    OpenCV(#[from]opencv::Error),
}


