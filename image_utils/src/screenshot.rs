use ndarray::{Array, Array3};
use opencv::core::AlgorithmHint;
use opencv::prelude::*;
use xcap::{Monitor};
use crate::consts::DEFAULT_ALGORITHM_HINT;
use crate::screenshot_error::ScreenshotError;

pub fn screenshot_to_ndarray(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<Array3<u8>, ScreenshotError> {
    let monitors = Monitor::all()?;
    let monitor = &monitors[0];
    let image = monitor.capture_region(x, y, width, height)?;

    let img_width = image.width() as usize;
    let img_height = image.height() as usize;
    let rgba_data = image.into_vec();

    let array = Array::from_shape_vec((img_height, img_width, 4), rgba_data)?;

    Ok(array)

}

/// 快速截图（不保存路径）
///
/// # 参数
/// - `x`: 截图区域左上角 X 坐标
/// - `y`: 截图区域左上角 Y 坐标
/// - `width`: 截图宽度
/// - `height`: 截图高度
///
/// # 返回
/// 返回 OpenCV Mat 格式的图像（BGR 格式）
///
/// # 示例
/// ```rust
/// use gjx_image_rs::screenshot_fast_no_save;
///
/// let img = screenshot_fast_no_save(100, 100, 800, 600)?;
/// ```
pub fn screenshot_to_mat(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<opencv::core::Mat, ScreenshotError> {
    let monitors = Monitor::all()?;
    if monitors.is_empty() {
        return Err(ScreenshotError::NoMonitorFound);
    }

    let monitor = &monitors[0];
    let image = monitor.capture_region(x, y, width, height)?;

    let img_width = image.width() as i32;
    let img_height = image.height() as i32;
    let rgba_data = image.into_vec();

    // 创建 RGBA Mat（OpenCV 内部会优化，from_slice 和 reshape 的开销很小）
    let mat = opencv::core::Mat::from_slice(&rgba_data)?;
    let mat = mat.reshape(4, img_height)?; // 4 通道 (RGBA)

    // 转换为 BGR
    let mut bgr_mat = opencv::core::Mat::default();
    opencv::imgproc::cvt_color(
        &mat,
        &mut bgr_mat,
        opencv::imgproc::COLOR_RGBA2BGR,
        0,
        DEFAULT_ALGORITHM_HINT
    )?;
    
    Ok(bgr_mat)
}

/// 截图并直接转换为灰度图（优化版，避免 BGR 中间转换）
///
/// # 参数
/// - `x`: 截图区域左上角 X 坐标
/// - `y`: 截图区域左上角 Y 坐标
/// - `width`: 截图宽度
/// - `height`: 截图高度
///
/// # 返回
/// 返回 OpenCV Mat 格式的灰度图像（单通道）
pub fn screenshot_to_mat_gray(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<opencv::core::Mat, ScreenshotError> {
    let monitors = Monitor::all()?;
    if monitors.is_empty() {
        return Err(ScreenshotError::NoMonitorFound);
    }

    let monitor = &monitors[0];
    let image = monitor.capture_region(x, y, width, height)?;

    let img_width = image.width() as i32;
    let img_height = image.height() as i32;
    let rgba_data = image.into_vec();

    // 创建 RGBA Mat（OpenCV 内部会优化，from_slice 和 reshape 的开销很小）
    let mat = opencv::core::Mat::from_slice(&rgba_data)?;
    let mat = mat.reshape(4, img_height)?; // 4 通道 (RGBA)

    // 直接从 RGBA 转换为灰度，避免 BGR 中间转换
    let mut gray_mat = opencv::core::Mat::default();
    opencv::imgproc::cvt_color(
        &mat,
        &mut gray_mat,
        opencv::imgproc::COLOR_RGBA2GRAY,
        0,
        DEFAULT_ALGORITHM_HINT
    )?;
    
    Ok(gray_mat)
}

/// 二值化截图
///
/// # 参数
/// - `x1`: 截图区域左上角 X 坐标
/// - `y1`: 截图区域左上角 Y 坐标
/// - `x2`: 截图区域右下角 X 坐标
/// - `y2`: 截图区域右下角 Y 坐标
pub fn screenshot_to_mat_binary(
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
) -> Result<Mat, ScreenshotError> {
    let width = (x2 - x1);
    let height = (y2 - y1);

    // 截图
    let img = screenshot_to_mat(x1, y1, width, height)?;

    // 转换为灰度图
    let mut gray = opencv::core::Mat::default();
    opencv::imgproc::cvt_color(&img, &mut gray, opencv::imgproc::COLOR_BGR2GRAY, 0, DEFAULT_ALGORITHM_HINT)?;

    // 二值化（使用 OTSU 方法）
    let mut binary = opencv::core::Mat::default();
    opencv::imgproc::threshold(
        &gray,
        &mut binary,
        0.0,
        255.0,
        opencv::imgproc::THRESH_BINARY | opencv::imgproc::THRESH_OTSU,
    )?;

    Ok(gray)
}
