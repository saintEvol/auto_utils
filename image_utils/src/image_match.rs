use std::time::Instant;
use std::path::Path;
use std::sync::Arc;
use opencv::core::{MatTraitConst};
use opencv::{imgcodecs, imgproc};
use opencv::prelude::MatTraitConstManual;
use rayon::prelude::*;
use crate::consts::DEFAULT_ALGORITHM_HINT;
use crate::image_match_error::ImageMatchError;
use crate::screenshot::{screenshot_to_mat, screenshot_to_mat_gray};
use crate::types::{MatchResult, Point};

/// 读取图像（兼容 aircv.imread）
///
/// # 参数
/// - `path`: 图像文件路径
///
/// # 返回
/// OpenCV Mat 格式的图像
pub fn read_image(path: &str) -> Result<opencv::core::Mat, ImageMatchError> {
    let img = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR)?;
    if img.empty() {
        return Err(ImageMatchError::CanNotReadImage(path.to_string()))
    }
    Ok(img)
}

/// 查找图片（优化版）- 返回布尔值
///
/// # 参数
/// - `x`: 截图区域左上角 X
/// - `y`: 截图区域左上角 Y
/// - `width`: 截图宽度
/// - `height`: 截图高度
/// - `image_path`: 模板图片路径
/// - `threshold`: 相似度阈值 (默认 0.75)
/// - `rgb`: 是否使用彩色匹配 (默认 true)
///
/// # 返回
/// 如果找到匹配返回 true，否则返回 false
///
/// # 示例
/// ```rust
/// use gjx_image_rs::find_image_optimized;
///
/// let found = find_image_optimized(100, 100, 800, 600, "template.png", 0.75, true)?;
/// if found {
///     println!("找到图片！");
/// }
/// ```
/// todo: 待优化，较python版本慢
pub fn find_image_optimized(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    image_path: &str,
    threshold: f64,
    rgb: bool,
) -> Result<bool, ImageMatchError> {
    // 读取模板（先读取，避免截图后等待）
    let now = Instant::now();
    let template = read_image(image_path)?;
    let cost = now.elapsed().as_micros();
    println!("[find_image_optimized]读取模板{cost} 微秒");

    // 截图 - 根据模式选择最优路径
    let now = Instant::now();
    let screenshot = if rgb {
        // 彩色模式：需要 BGR
        screenshot_to_mat(x as u32, y as u32, width, height)?
    } else {
        // 灰度模式：直接从 RGBA 转灰度，避免 BGR 中间转换
        screenshot_to_mat_gray(x as u32, y as u32, width, height)?
    };
    let cost = now.elapsed().as_micros();
    println!("[find_image_optimized]截图用时{cost} 微秒");

    // 匹配 - 只检查是否存在匹配，不需要提取所有结果
    let now = Instant::now();
    let found = find_template_exists(&screenshot, &template, threshold, rgb)?;
    let cost = now.elapsed().as_micros();
    println!("[find_image_optimized]匹配模板{cost} 微秒");

    Ok(found)
}

/// 查找图片（坐标版优化版）- 返回第一个匹配的中心点坐标
///
/// # 参数
/// - `x`: 截图区域左上角 X
/// - `y`: 截图区域左上角 Y
/// - `width`: 截图宽度
/// - `height`: 截图高度
/// - `image_path`: 模板图片路径
/// - `threshold`: 相似度阈值 (默认 0.75)
/// - `rgb`: 是否使用彩色匹配 (默认 true)
///
/// # 返回
/// 如果找到匹配，返回绝对坐标 (中心点 x, 中心点 y)，否则返回 (0, 0)
///
/// # 示例
/// ```rust
/// use gjx_image_rs::find_image_optimized_coord;
///
/// let (x, y) = find_image_optimized_coord(100, 100, 800, 600, "template.png", 0.75, true)?;
/// if x != 0 || y != 0 {
///     println!("找到图片，中心点坐标: ({}, {})", x, y);
/// }
/// ```
/// todo: 待优化，较pyton版本慢
pub fn find_image_optimized_coord(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    image_path: &str,
    threshold: f64,
    rgb: bool,
) -> Result<(i32, i32), ImageMatchError> {
    // 读取模板（先读取，避免截图后等待）
    let template = read_image(image_path)?;

    // 截图 - 根据模式选择最优路径
    let screenshot = if rgb {
        // 彩色模式：需要 BGR
        screenshot_to_mat(x as u32, y as u32, width, height)?
    } else {
        // 灰度模式：直接从 RGBA 转灰度，避免 BGR 中间转换
        screenshot_to_mat_gray(x as u32, y as u32, width, height)?
    };

    // 使用与 find_images_optimized_coords 相同的方式：调用 find_all_template 获取所有匹配
    // 然后取第一个（置信度最高的）匹配，确保坐标计算方式一致
    let matches = find_all_template(&screenshot, &template, threshold, rgb)?;
    
    if let Some(first_match) = matches.first() {
        // 使用与 find_images_optimized_coords 相同的坐标提取方式
        let center_x = (*first_match.result.x()).round() as i32;
        let center_y = (*first_match.result.y()).round() as i32;
        Ok((x + center_x, y + center_y))
    } else {
        // 未找到匹配
        Ok((0, 0))
    }
}

/// 查找多图片（坐标版多目标）- 返回所有匹配的中心点坐标
///
/// # 参数
/// - `x`: 截图区域左上角 X
/// - `y`: 截图区域左上角 Y
/// - `width`: 截图宽度
/// - `height`: 截图高度
/// - `image_paths`: 模板图片路径列表
/// - `threshold`: 相似度阈值 (默认 0.75)
/// - `rgb`: 是否使用彩色匹配 (默认 true)
///
/// # 返回
/// 返回所有找到的匹配坐标列表，每个元素为 (中心点 x, 中心点 y)
///
/// # 示例
/// ```rust
/// use gjx_image_rs::find_images_optimized_coords;
///
/// let paths = vec!["template1.png", "template2.png"];
/// let coords = find_images_optimized_coords(100, 100, 800, 600, &paths, 0.75, true)?;
/// for (x, y) in coords {
///     println!("找到图片，中心点坐标: ({}, {})", x, y);
/// }
/// ```
/// todo: 待优化，较python慢
pub fn find_images_optimized_coords(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    image_paths: &[&str],
    threshold: f64,
    rgb: bool,
) -> Result<Vec<(i32, i32)>, ImageMatchError> {
    if image_paths.is_empty() {
        return Ok(Vec::new());
    }

    // 先截图一次，所有模板共享
    let screenshot = if rgb {
        screenshot_to_mat(x as u32, y as u32, width, height)?
    } else {
        screenshot_to_mat_gray(x as u32, y as u32, width, height)?
    };

    let mut all_coords = Vec::new();

    // 对每个模板进行匹配
    for image_path in image_paths {
        let template = read_image(image_path)?;
        
        // 获取模板尺寸，用于判断重叠
        let template_size = template.size()?;
        let template_w = template_size.width;
        let template_h = template_size.height;
        
        // 查找所有匹配
        let matches = find_all_template(&screenshot, &template, threshold, rgb)?;
        
        // 使用非极大值抑制（NMS）过滤重叠的匹配
        // 由于 matches 已按置信度降序排序，我们遍历并只保留不重叠的匹配
        let mut filtered_coords = Vec::new();
        
        // 使用模板尺寸作为最小距离阈值（如果两个匹配距离小于模板尺寸，认为是同一个）
        let min_distance = template_w.max(template_h) as i32;
        
        for match_result in matches {
            let center_x = (*match_result.result.x()).round() as i32;
            let center_y = (*match_result.result.y()).round() as i32;
            let abs_x = x + center_x;
            let abs_y = y + center_y;
            
            // 检查是否与已有匹配重叠
            let mut is_overlapping = false;
            
            for (existing_x, existing_y) in &filtered_coords {
                // 使用曼哈顿距离判断重叠（更快）
                let dx = i32::abs(abs_x - *existing_x);
                let dy = i32::abs(abs_y - *existing_y);
                
                // 如果 X 和 Y 方向的距离都小于模板尺寸，认为是重叠
                if dx < min_distance && dy < min_distance {
                    is_overlapping = true;
                    break;
                }
            }
            
            // 如果不重叠，添加到结果列表
            if !is_overlapping {
                filtered_coords.push((abs_x, abs_y));
            }
        }
        
        all_coords.extend(filtered_coords);
    }

    Ok(all_coords)
}

/// 查找所有模板匹配（兼容 aircv.find_all_template）
///
/// # 参数
/// - `imgsrc`: 源图像（OpenCV Mat）
/// - `imgobj`: 模板图像（OpenCV Mat）
/// - `confidence`: 相似度阈值 (0.0-1.0)
/// - `rgb`: 是否使用彩色匹配（true=彩色，false=灰度）
///
/// # 返回
/// 匹配结果列表
///
/// # 示例
/// ```rust
/// use gjx_image_rs::{imread, find_all_template};
///
/// let src = imread("screenshot.png")?;
/// let template = imread("template.png")?;
/// let results = find_all_template(&src, &template, 0.8, true)?;
///
/// for result in results {
///     println!("找到匹配: 置信度={}, 中心点=({}, {})",
///              result.confidence, result.result.0, result.result.1);
/// }
/// ```
pub fn find_all_template(
    imgsrc: &opencv::core::Mat,
    imgobj: &opencv::core::Mat,
    confidence: f64,
    rgb: bool,
) -> Result<Vec<MatchResult<i32>>, ImageMatchError> {
    let mut result_mat = opencv::core::Mat::default();

    if rgb {
        // 彩色模式直接匹配
        imgproc::match_template(
            imgsrc,
            imgobj,
            &mut result_mat,
            imgproc::TM_CCOEFF_NORMED,
            &opencv::core::Mat::default(),
        )?;
    } else {
        // 灰度模式
        // 如果源图像已经是灰度图（单通道），直接使用；否则转换
        let gray_src = if imgsrc.channels() == 1 {
            imgsrc.clone()
        } else {
            let mut gray = opencv::core::Mat::default();
            imgproc::cvt_color(imgsrc, &mut gray, imgproc::COLOR_BGR2GRAY, 0, DEFAULT_ALGORITHM_HINT)?;
            gray
        };
        
        // 模板图像转换为灰度
        let mut gray_obj = opencv::core::Mat::default();
        imgproc::cvt_color(imgobj, &mut gray_obj, imgproc::COLOR_BGR2GRAY, 0, DEFAULT_ALGORITHM_HINT)?;

        imgproc::match_template(
            &gray_src,
            &gray_obj,
            &mut result_mat,
            imgproc::TM_CCOEFF_NORMED,
            &opencv::core::Mat::default(),
        )?;
    }

    extract_matches(&result_mat, imgobj, confidence)
}

/// 检查模板是否存在（优化版，只返回布尔值，找到第一个匹配就返回）
///
/// # 参数
/// - `imgsrc`: 源图像（OpenCV Mat）
/// - `imgobj`: 模板图像（OpenCV Mat）
/// - `confidence`: 相似度阈值 (0.0-1.0)
/// - `rgb`: 是否使用彩色匹配（true=彩色，false=灰度）
///
/// # 返回
/// 如果找到匹配返回 true，否则返回 false
fn find_template_exists(
    imgsrc: &opencv::core::Mat,
    imgobj: &opencv::core::Mat,
    confidence: f64,
    rgb: bool,
) -> Result<bool, ImageMatchError> {
    let mut result_mat = opencv::core::Mat::default();

    if rgb {
        // 彩色模式直接匹配
        imgproc::match_template(
            imgsrc,
            imgobj,
            &mut result_mat,
            imgproc::TM_CCOEFF_NORMED,
            &opencv::core::Mat::default(),
        )?;
    } else {
        // 灰度模式
        // 如果源图像已经是灰度图（单通道），直接使用；否则转换
        let gray_src = if imgsrc.channels() == 1 {
            imgsrc.clone()
        } else {
            let mut gray = opencv::core::Mat::default();
            imgproc::cvt_color(imgsrc, &mut gray, imgproc::COLOR_BGR2GRAY, 0, DEFAULT_ALGORITHM_HINT)?;
            gray
        };
        
        // 模板图像转换为灰度
        let mut gray_obj = opencv::core::Mat::default();
        imgproc::cvt_color(imgobj, &mut gray_obj, imgproc::COLOR_BGR2GRAY, 0, DEFAULT_ALGORITHM_HINT)?;

        imgproc::match_template(
            &gray_src,
            &gray_obj,
            &mut result_mat,
            imgproc::TM_CCOEFF_NORMED,
            &opencv::core::Mat::default(),
        )?;
    }

    // 快速检查：找到第一个超过阈值的匹配就返回
    // 使用更高效的方式访问数据
    let rows = result_mat.rows();
    let cols = result_mat.cols();
    let threshold_f32 = confidence as f32;
    
    // 尝试使用连续内存访问（如果 Mat 是连续的）
    if result_mat.is_continuous() {
        unsafe {
            let data_ptr = result_mat.ptr_2d(0, 0)? as *const f32;
            let total_pixels = (rows * cols) as usize;
            
            for i in 0..total_pixels {
                let confidence_val = *data_ptr.add(i);
                if confidence_val >= threshold_f32 {
                    return Ok(true);
                }
            }
        }
    } else {
        // 如果不是连续的，按行访问
        for y in 0..rows {
            unsafe {
                let row_ptr = result_mat.ptr_2d(y, 0)? as *const f32;
                for x in 0..cols {
                    let confidence_val = *row_ptr.add(x as usize);
                    if confidence_val >= threshold_f32 {
                        return Ok(true);
                    }
                }
            }
        }
    }

    Ok(false)
}

/// 查找模板并返回第一个匹配的坐标（优化版）
///
/// # 参数
/// - `imgsrc`: 源图像（OpenCV Mat）
/// - `imgobj`: 模板图像（OpenCV Mat）
/// - `confidence`: 相似度阈值 (0.0-1.0)
/// - `rgb`: 是否使用彩色匹配（true=彩色，false=灰度）
/// - `offset_x`: 截图区域的 X 偏移（用于计算绝对坐标）
/// - `offset_y`: 截图区域的 Y 偏移（用于计算绝对坐标）
///
/// # 返回
/// 如果找到匹配，返回绝对坐标 (中心点 x, 中心点 y)，否则返回 (0, 0)
pub fn find_template_coord(
    imgsrc: &opencv::core::Mat,
    imgobj: &opencv::core::Mat,
    confidence: f64,
    rgb: bool,
    offset_x: i32,
    offset_y: i32,
) -> Result<(i32, i32), ImageMatchError> {
    let mut result_mat = opencv::core::Mat::default();

    if rgb {
        // 彩色模式直接匹配
        imgproc::match_template(
            imgsrc,
            imgobj,
            &mut result_mat,
            imgproc::TM_CCOEFF_NORMED,
            &opencv::core::Mat::default(),
        )?;
    } else {
        // 灰度模式
        // 如果源图像已经是灰度图（单通道），直接使用；否则转换
        let gray_src = if imgsrc.channels() == 1 {
            imgsrc.clone()
        } else {
            let mut gray = opencv::core::Mat::default();
            imgproc::cvt_color(imgsrc, &mut gray, imgproc::COLOR_BGR2GRAY, 0, DEFAULT_ALGORITHM_HINT)?;
            gray
        };
        
        // 模板图像转换为灰度
        let mut gray_obj = opencv::core::Mat::default();
        imgproc::cvt_color(imgobj, &mut gray_obj, imgproc::COLOR_BGR2GRAY, 0, DEFAULT_ALGORITHM_HINT)?;

        imgproc::match_template(
            &gray_src,
            &gray_obj,
            &mut result_mat,
            imgproc::TM_CCOEFF_NORMED,
            &opencv::core::Mat::default(),
        )?;
    }

    // 获取模板尺寸，用于计算中心点
    let template_size = imgobj.size()?;
    let template_w = template_size.width;
    let template_h = template_size.height;

    // 快速查找：找到第一个超过阈值的匹配就返回坐标
    let rows = result_mat.rows();
    let cols = result_mat.cols();
    let threshold_f32 = confidence as f32;
    
    // 尝试使用连续内存访问（如果 Mat 是连续的）
    if result_mat.is_continuous() {
        unsafe {
            let data_ptr = result_mat.ptr_2d(0, 0)? as *const f32;
            let total_pixels = (rows * cols) as usize;
            
            for i in 0..total_pixels {
                let confidence_val = *data_ptr.add(i);
                if confidence_val >= threshold_f32 {
                    // 计算在结果矩阵中的位置
                    let y = (i / cols as usize) as i32;
                    let x = (i % cols as usize) as i32;
                    
                    // 计算中心点坐标（相对于截图区域）
                    // 使用浮点数计算然后四舍五入（与 extract_matches 保持一致）
                    let center_x = (x as f64 + template_w as f64 / 2.0).round() as i32;
                    let center_y = (y as f64 + template_h as f64 / 2.0).round() as i32;
                    
                    // 转换为绝对坐标
                    return Ok((offset_x + center_x, offset_y + center_y));
                }
            }
        }
    } else {
        // 如果不是连续的，按行访问
        for y in 0..rows {
            unsafe {
                let row_ptr = result_mat.ptr_2d(y, 0)? as *const f32;
                for x in 0..cols {
                    let confidence_val = *row_ptr.add(x as usize);
                    if confidence_val >= threshold_f32 {
                        // 计算中心点坐标（相对于截图区域）
                        // 使用浮点数计算然后四舍五入（与 extract_matches 保持一致）
                        let center_x = (x as f64 + template_w as f64 / 2.0).round() as i32;
                        let center_y = (y as f64 + template_h as f64 / 2.0).round() as i32;
                        
                        // 转换为绝对坐标
                        return Ok((offset_x + center_x, offset_y + center_y));
                    }
                }
            }
        }
    }

    // 未找到匹配
    Ok((0, 0))
}

/// 确保图像是3通道的CV_8U类型
// fn ensure_3channels_u8(mat: &opencv::core::Mat) -> opencv::Result<opencv::core::Mat> {
//     let mut result = mat.clone();
//
//     // 检查深度
//     if result.depth() != opencv::core::CV_8U {
//         result.convert_to(&mut result, opencv::core::CV_8U, 1.0, 0.0)?;
//     }
//
//     // 检查通道数
//     if result.channels() != 3 {
//         let mut converted = opencv::core::Mat::default();
//         match result.channels() {
//             1 => imgproc::cvt_color(&result, &mut converted, imgproc::COLOR_GRAY2BGR, 0, DEFAULT_ALGORITHM_HINT)?,
//             4 => imgproc::cvt_color(&result, &mut converted, imgproc::COLOR_BGRA2BGR, 0, DEFAULT_ALGORITHM_HINT)?,
//             _ => return Err(opencv::Error::new(
//                 opencv::core::StsError,
//                 format!("Unsupported number of channels: {}", result.channels())
//             )),
//         }
//         result = converted;
//     }
//
//     Ok(result)
// }

/// 从匹配结果矩阵中提取所有匹配点
fn extract_matches(
    match_result: &opencv::core::Mat,
    template: &opencv::core::Mat,
    threshold: f64,
) -> Result<Vec<MatchResult<i32>>, ImageMatchError> {
    let mut matches = Vec::new();

    let template_size = template.size()?;
    let template_w = template_size.width;
    let template_h = template_size.height;

    let rows = match_result.rows();
    let cols = match_result.cols();

    // 遍历所有像素，找到超过阈值的匹配
    for y in 0..rows {
        for x in 0..cols {
            unsafe {
                let confidence_val = *match_result.at_2d_unchecked::<f32>(y, x)?;

                if confidence_val as f64 >= threshold {
                    // 计算中心点
                    let center_x = x as f64 + template_w as f64 / 2.0;
                    let center_y = y as f64 + template_h as f64 / 2.0;

                    // 计算四个角点
                    let rectangle = [
                        Point::new(x, y),                                    // 左上
                        Point::new(x, y + template_h),                       // 左下
                        Point::new(x + template_w, y),                       // 右上
                        Point::new(x + template_w, y + template_h),         // 右下
                    ];

                    matches.push(MatchResult {
                        confidence: confidence_val as f64,
                        rectangle,
                        result: Point::new(center_x, center_y),
                    });
                }
            }
        }
    }

    // 按置信度降序排序
    matches.sort_by(|a, b| {
        b.confidence.partial_cmp(&a.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(matches)
}

/// 找字_图库图片找字_find_all_template_线程版（Rust 实现）
/// 
/// 在指定区域中查找数字 0-9，使用并行处理提高性能
/// 
/// # 参数
/// - `x1`: 截图区域左上角 X 坐标
/// - `y1`: 截图区域左上角 Y 坐标
/// - `width`: 截图宽度
/// - `height`: 截图高度
/// - `library_path`: 图库路径（包含 0.bmp 到 9.bmp 的文件夹）
/// - `threshold`: 相似度阈值（默认 0.9）
/// 
/// # 返回
/// 识别到的数字字符串（按从左到右的顺序）
/// 
/// # 示例
/// ```rust
/// use image_utils::image_match::find_characters_from_library_threaded;
/// 
/// let result = find_characters_from_library_threaded(
///     100, 100,      // 截图区域
///     800, 600,      // 截图尺寸
///     "C:\\path\\to\\library",  // 图库路径
///     0.9,           // 相似度阈值
/// )?;
/// 
/// println!("识别结果: {}", result);
/// ```
/// todo: 性能慢于python
pub fn find_characters_from_library_threaded(
    x1: i32,
    y1: i32,
    width: u32,
    height: u32,
    library_path: &str,
    threshold: f64,
) -> Result<String, ImageMatchError> {
    // 截图（使用灰度模式，与 Python 版本保持一致）
    let screenshot = screenshot_to_mat_gray(x1 as u32, y1 as u32, width, height)?;
    let screenshot_arc = Arc::new(screenshot);
    let library_path = Arc::new(library_path.to_string());

    // 使用并行处理查找所有数字（0-9）
    let results: Vec<(f64, u8)> = (0..10)
        .into_par_iter()
        .flat_map(|digit| {
            // 构建模板图片路径：library_path + "\\" + digit + ".bmp"
            let template_path = Path::new(library_path.as_str())
                .join(format!("{}.bmp", digit));
            
            let template_path_str = match template_path.to_str() {
                Some(s) => s,
                None => return Vec::new(),
            };
            
            // 读取模板图片
            let template = match read_image(template_path_str) {
                Ok(t) => t,
                Err(_) => return Vec::new(), // 如果文件不存在，跳过
            };

            // 在截图中查找所有匹配
            let matches = match find_all_template(&screenshot_arc, &template, threshold, false) {
                Ok(m) => m,
                Err(_) => return Vec::new(),
            };

            // 收集所有匹配结果：[(x坐标, 数字)]
            matches
                .into_iter()
                .map(|match_result| {
                    let x = *match_result.result.x();
                    (x, digit as u8)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // 按 X 坐标排序
    let mut sorted_results = results;
    sorted_results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // 生成字符串
    let content_string: String = sorted_results
        .iter()
        .map(|(_, digit)| char::from(b'0' + *digit))
        .collect();

    Ok(content_string)
}