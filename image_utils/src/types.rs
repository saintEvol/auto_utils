//! 类型定义

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    pub fn x(&self) -> &T {
        &self.x
    }
    
    pub fn y(&self) -> &T {
        &self.y
    }
}

/// 匹配结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult<T> {
    /// 相似度 (0.0-1.0)
    pub confidence: f64,
    /// 矩形四个角点坐标 [(x1,y1), (x2,y2), (x3,y3), (x4,y4)]
    pub rectangle: [Point<T>; 4],
    /// 中心点坐标 (x, y)
    pub result: Point<f64>,
}

/// RGB 颜色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_tuple(rgb: (u8, u8, u8)) -> Self {
        Self {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
        }
    }

    pub fn to_tuple(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

/// 图像数据（OpenCV Mat 的封装）
pub type ImageMat = opencv::core::Mat;


