use opencv::prelude::{MatTraitConst, MatTraitConstManual};
use crate::image_match_error::ImageMatchError;
use crate::screenshot::{screenshot_to_mat};

/// 计算两个颜色之间的差异
///
/// # 参数
/// - `color1`: 颜色1 (R, G, B)
/// - `color2`: 颜色2 (R, G, B)
///
/// # 返回
/// 颜色差异值（曼哈顿距离）
pub fn calculate_color_difference(color1: (u8, u8, u8), color2: (u8, u8, u8)) -> u32 {
    (color1.0 as i32 - color2.0 as i32).abs() as u32 +
        (color1.1 as i32 - color2.1 as i32).abs() as u32 +
        (color1.2 as i32 - color2.2 as i32).abs() as u32
}

/// 屏幕点找色（优化版）
///
/// # 参数
/// - `x`: 屏幕 X 坐标
/// - `y`: 屏幕 Y 坐标
/// - `target_rgb`: 目标颜色 (R, G, B)
/// - `tolerance`: 容差值
///
/// # 返回
/// 如果颜色匹配返回 true，否则返回 false
///
/// # 示例
/// ```rust
/// use gjx_image_rs::find_color_at_point;
///
/// let found = find_color_at_point(100, 100, (255, 0, 0), 10)?;
/// ```
pub fn find_color_at_point(
    x: i32,
    y: i32,
    target_rgb: (u8, u8, u8),
    tolerance: u32,
) -> Result<bool, ImageMatchError> {
    // 截取 1x1 像素区域
    let img = screenshot_to_mat(x as u32, y as u32, 1, 1)?;

    // 获取像素颜色（OpenCV 是 BGR 格式）
    unsafe {
        let pixel = *img.at_2d_unchecked::<opencv::core::Vec3b>(0, 0)?;
        let bgr = (pixel[0], pixel[1], pixel[2]);

        // 转换为 RGB
        let rgb = (bgr.2, bgr.1, bgr.0);

        let diff = calculate_color_difference(rgb, target_rgb);
        Ok(diff <= tolerance)
    }
}

/// 屏幕区域找色（优化版）- 返回布尔值
///
/// # 参数
/// - `x1`: 区域左上角 X 坐标
/// - `y1`: 区域左上角 Y 坐标
/// - `x2`: 区域右下角 X 坐标（注意：Python 版本中这个参数实际是宽度）
/// - `y2`: 区域右下角 Y 坐标（注意：Python 版本中这个参数实际是高度）
/// - `target_rgb`: 目标颜色 (R, G, B)
/// - `tolerance`: 容差值
///
/// # 返回
/// 如果找到匹配颜色返回 true，否则返回 false
///
/// # 注意
/// Python 版本的参数命名有歧义，这里按照实际使用情况：
/// - 如果 x2, y2 是坐标，则 width = x2 - x1, height = y2 - y1
/// - 如果 x2, y2 是宽高，则直接使用
///
/// 根据代码分析，Python 版本实际传入的是宽高，所以这里按宽高处理
///
/// # 示例
/// ```rust
/// use gjx_image_rs::find_color_in_region;
///
/// let found = find_color_in_region(100, 100, 200, 150, (255, 0, 0), 10)?;
/// ```
pub fn find_color_in_region(
    x1: u32,
    y1: u32,
    width: u32,  // 注意：对应 Python 的 x2 参数（实际是宽度）
    height: u32, // 注意：对应 Python 的 y2 参数（实际是高度）
    target_rgb: (u8, u8, u8),
    tolerance: u32,
) -> anyhow::Result<bool> {
    // 截图
    let img = screenshot_to_mat(x1, y1, width, height)?;

    // 转换为 RGB（OpenCV 默认是 BGR）
    let rows = img.rows();
    let cols = img.cols();

    // 遍历所有像素
    for y in 0..rows {
        for x in 0..cols {
            unsafe {
                let pixel = *img.at_2d_unchecked::<opencv::core::Vec3b>(y, x)?;
                let bgr = (pixel[0], pixel[1], pixel[2]);

                // 转换为 RGB
                let rgb = (bgr.2, bgr.1, bgr.0);

                let diff = calculate_color_difference(rgb, target_rgb);
                if diff <= tolerance {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}


/// 屏幕区域找色（坐标版优化版）- 返回坐标
///
/// # 参数
/// - `x1`: 区域左上角 X 坐标
/// - `y1`: 区域左上角 Y 坐标
/// - `width`: 区域宽度
/// - `height`: 区域高度
/// - `target_rgb`: 目标颜色 (R, G, B)
/// - `tolerance`: 容差值
///
/// # 返回
/// 如果找到，返回绝对坐标 (x, y)，否则返回 (0, 0)
///
/// # 示例
/// ```rust
/// use gjx_image_rs::find_color_in_region_coord;
///
/// let (x, y) = find_color_in_region_coord(100, 100, 200, 150, (255, 0, 0), 10)?;
/// if x != 0 || y != 0 {
///     println!("找到颜色，坐标: ({}, {})", x, y);
/// }
/// ```
pub fn find_color_in_region_coord(
    x1: u32,
    y1: u32,
    width: u32,
    height: u32,
    target_rgb: (u8, u8, u8),
    tolerance: u32,
) -> anyhow::Result<(u32, u32)> {
    // 截图
    let img = screenshot_to_mat(x1, y1, width, height)?;

    let rows = img.rows();
    let cols = img.cols();

    // 遍历所有像素
    for y in 0..rows {
        for x in 0..cols {
            unsafe {
                let pixel = *img.at_2d_unchecked::<opencv::core::Vec3b>(y, x)?;
                let bgr = (pixel[0], pixel[1], pixel[2]);

                // 转换为 RGB
                let rgb = (bgr.2, bgr.1, bgr.0);

                let diff = calculate_color_difference(rgb, target_rgb);
                if diff <= tolerance {
                    // 返回绝对坐标
                    return Ok((x1 + x as u32, y1 + y as u32));
                }
            }
        }
    }

    Ok((0, 0))
}