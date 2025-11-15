
use ndarray::Array3;
use opencv::core::MatTrait;
use opencv::imgcodecs;
use xcap::image::{RgbImage, RgbaImage};
use crate::consts::DEFAULT_ALGORITHM_HINT;

pub fn save_array3_fast(array: &Array3<u8>, path: &str) -> anyhow::Result<()> {
    let (height, width, channels) = (
        array.shape()[0],
        array.shape()[1],
        array.shape()[2],
    );

    // 确保数据是连续的
    if !array.is_standard_layout() {
        anyhow::bail!("数组不是标准布局，无法直接保存");
    }

    match channels {
        3 => {
            // 直接使用底层数据创建 RGB 图像
            let data = array.as_slice().unwrap();
            let img = RgbImage::from_raw(width as u32, height as u32, data.to_vec())
                .ok_or_else(|| anyhow::anyhow!("创建 RGB 图像失败"))?;
            img.save(path)?;
        }
        4 => {
            // 直接使用底层数据创建 RGBA 图像
            let data = array.as_slice().unwrap();
            let img = RgbaImage::from_raw(width as u32, height as u32, data.to_vec())
                .ok_or_else(|| anyhow::anyhow!("创建 RGBA 图像失败"))?;
            img.save(path)?;
        }
        _ => anyhow::bail!("不支持的通道数: {}", channels),
    }

    println!("图像已保存: {}", path);
    Ok(())
}

pub fn save_array3_via_opencv(array: &Array3<u8>, filename: &str) -> anyhow::Result<()> {
    let (height, width, channels) = (
        array.shape()[0] as i32,
        array.shape()[1] as i32,
        array.shape()[2],
    );

    // 手动创建 Mat 并填充数据
    let mut mat = unsafe {opencv::core::Mat::new_rows_cols(height, width, opencv::core::CV_8UC(channels as i32))?};

    // 手动复制数据
    for y in 0..height {
        for x in 0..width {
            match channels {
                3 => {
                    let b = array[[y as usize, x as usize, 0]];
                    let g = array[[y as usize, x as usize, 1]];
                    let r = array[[y as usize, x as usize, 2]];
                    *mat.at_2d_mut::<opencv::core::Vec3b>(y, x)? =
                        opencv::core::Vec3b::from([b, g, r]);
                }
                4 => {
                    let b = array[[y as usize, x as usize, 0]];
                    let g = array[[y as usize, x as usize, 1]];
                    let r = array[[y as usize, x as usize, 2]];
                    let a = array[[y as usize, x as usize, 3]];
                    *mat.at_2d_mut::<opencv::core::Vec4b>(y, x)? =
                        opencv::core::Vec4b::from([b, g, r, a]);
                }
                _ => anyhow::bail!("不支持的通道数: {}", channels),
            }
        }
    }

    println!("保存之前1");
    // 如果是 RGBA，转换为 BGR 再保存（OpenCV 的 imwrite 对 RGBA 支持不好）
    let save_mat = if channels == 4 {
        let mut bgr_mat = opencv::core::Mat::default();
        opencv::imgproc::cvt_color(&mat, &mut bgr_mat, opencv::imgproc::COLOR_RGBA2BGR, 0, DEFAULT_ALGORITHM_HINT)?;
        bgr_mat
    } else {
        mat
    };

    imgcodecs::imwrite(filename, &save_mat, &opencv::core::Vector::new())?;
    println!("通过 OpenCV 保存: {}", filename);
    Ok(())
}