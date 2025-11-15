use tokio::main;
use image_utils::color_detection::{find_color_at_point, find_color_in_region, find_color_in_region_coord};
use image_utils::image_match::{find_image_optimized, find_image_optimized_coord};
use image_utils::saving::{save_array3_fast, save_array3_via_opencv};

#[main]
pub async fn main() {
    test_screenshot();
    test_color_detection();
    test_image_match();
}

fn test_screenshot() {
    let now = std::time::Instant::now();
    let r = image_utils::screenshot::screenshot_to_ndarray(0, 0, 2560, 1440).unwrap();
    // let r = image_utils::screenshot::screenshot_binary(0, 0, 2560, 1440).unwrap();
    // let r = image_utils::screenshot::screenshot_fast_no_save(0, 0, 300, 300).unwrap();
    save_array3_fast(&r, "./screen.png").unwrap();
    let cost = now.elapsed().as_micros();
    println!("截屏花费: {:?} 微秒", cost);
    // opencv::imgcodecs::imwrite("./screen.png", &r, &opencv::core::Vector::new()).unwrap();
}

fn test_color_detection() {
    let start = std::time::Instant::now();
    let ret = find_color_at_point(58, 128, (43, 45, 48), 10).unwrap();

    let cost = start.elapsed().as_micros();
    println!("在指定点找点花费: {:?}, 结果: {ret}", cost);

    let start = std::time::Instant::now();
    let ret = find_color_in_region(0, 0, 60, 200, (43, 45, 48), 10).unwrap();
    let cost = start.elapsed().as_micros();
    println!("在指定区域找点花费: {:?}, 结果: {ret}", cost);

    let start = std::time::Instant::now();
    let ret = find_color_in_region_coord(0, 0, 200, 400, (180, 142, 136), 10).unwrap();
    let cost = start.elapsed().as_micros();
    println!("在指定区域找点的坐标点花费: {}, 结果: {ret:?}", cost);
}

fn test_image_match() {
    let start = std::time::Instant::now();
    let ret = find_image_optimized(0, 0, 2560, 1440, "./xl.png", 0.8, false).unwrap();
    let cost = start.elapsed().as_micros();
    println!("在区域找图花费：{cost}, 结果: {ret}");
    
    // 测试坐标版
    let start = std::time::Instant::now();
    let (x, y) = find_image_optimized_coord(0, 0, 2560, 1440, "./xl.png", 0.8, false).unwrap();
    let cost = start.elapsed().as_micros();
    if x != 0 || y != 0 {
        println!("在区域找图（坐标版）花费：{cost} 微秒, 找到坐标: ({}, {})", x, y);
    } else {
        println!("在区域找图（坐标版）花费：{cost} 微秒, 未找到");
    }
}