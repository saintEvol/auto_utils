use ndarray::Array3;
use opencv::prelude::{Mat, MatTraitConst};
use opencv::imgproc;
use crate::consts::DEFAULT_ALGORITHM_HINT;

/// 将 ndarray 转换为 OpenCV Mat
///
/// # 参数
/// - `arr`: ndarray::Array3<u8>，形状为 [height, width, channels]
///
/// # 返回
/// OpenCV Mat（BGR 格式，3通道）
pub fn ndarray_to_mat(arr: &mut Array3<u8>) -> anyhow::Result<opencv::core::Mat> {
    let (height, _width, channels) = arr.dim();

    // 确保数组在内存中是连续的
    debug_assert!(arr.is_standard_layout());

    // 将数组数据转换为切片
    let data = arr.as_slice_mut().unwrap();

    // 创建 Mat 对象
    let mat = Mat::from_slice_mut(data).map(|mat| {
        mat.reshape(channels as i32, height as i32).unwrap().clone_pointee()
    })?;

    // 如果输入是 RGBA（4通道），转换为 BGR（3通道）
    // screenshot_to_ndarray 返回的是 RGBA 格式，需要转换为 BGR 以匹配 OpenCV 的标准格式
    if channels == 4 {
        let mut bgr_mat = opencv::core::Mat::default();
        imgproc::cvt_color(
            &mat,
            &mut bgr_mat,
            imgproc::COLOR_RGBA2BGR,
            0,
            DEFAULT_ALGORITHM_HINT,
        )?;
        Ok(bgr_mat)
    } else {
        // 其他通道数，直接返回（通常不会到达这里，因为 screenshot_to_ndarray 返回 4 通道）
        Ok(mat)
    }
    // let (height, width, channels) = arr.dim();
    //
    // // 确保数据是连续的
    // let arr = arr.as_standard_layout();
    //
    // // 创建 Mat（Mat 会复制数据）
    // let mat = unsafe {
    //     opencv::core::Mat::new_rows_cols_with_data(
    //         height as i32,
    //         width as i32,
    //         match channels {
    //             1 => opencv::core::CV_8UC1,
    //             3 => opencv::core::CV_8UC3,
    //             4 => opencv::core::CV_8UC4,
    //             _ => anyhow::bail!("不支持的通道数: {}", channels),
    //         },
    //         arr.as_ptr() as *mut std::ffi::c_void,
    //         opencv::core::Mat_AUTO_STEP,
    //     )?
    // };
    //
    // // 克隆 Mat 以确保数据所有权
    // Ok(mat.try_clone()?)
}