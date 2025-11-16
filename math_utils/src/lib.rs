use std::f64::consts::PI;

/// 旋转方向枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationDirection {
    /// 左转
    Left,
    /// 右转
    Right,
}

impl RotationDirection {
    /// 转换为字符串（中文）
    pub fn to_string_cn(&self) -> &'static str {
        match self {
            RotationDirection::Left => "左",
            RotationDirection::Right => "右",
        }
    }
}

/// 计算两点之间的欧几里得距离（直线距离）
///
/// # 参数
/// - `x1`: 第一个点的 X 坐标
/// - `y1`: 第一个点的 Y 坐标
/// - `x2`: 第二个点的 X 坐标
/// - `y2`: 第二个点的 Y 坐标
///
/// # 返回
/// 两点之间的欧几里得距离
///
/// # 示例
/// ```rust
/// use math_utils::calculate_distance;
///
/// let distance = calculate_distance(0.0, 0.0, 3.0, 4.0);
/// assert_eq!(distance, 5.0); // 3-4-5 直角三角形
/// ```
pub fn calculate_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

/// 计算两点之间的欧几里得距离（整数坐标版本）
///
/// # 参数
/// - `x1`: 第一个点的 X 坐标
/// - `y1`: 第一个点的 Y 坐标
/// - `x2`: 第二个点的 X 坐标
/// - `y2`: 第二个点的 Y 坐标
///
/// # 返回
/// 两点之间的欧几里得距离
///
/// # 示例
/// ```rust
/// use math_utils::calculate_distance_i32;
///
/// let distance = calculate_distance_i32(0, 0, 3, 4);
/// assert_eq!(distance, 5.0); // 3-4-5 直角三角形
/// ```
pub fn calculate_distance_i32(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
    let dx = (x2 - x1) as f64;
    let dy = (y2 - y1) as f64;
    (dx * dx + dy * dy).sqrt()
}

/// 根据当前坐标和路径数组，生成一个新的路径数组，从距离当前坐标最近的点开始
///
/// # 参数
/// - `path_array`: 路径数组，包含多个坐标点
/// - `current_coord`: 当前坐标 (x, y)
///
/// # 返回
/// 从距离当前坐标最近的点开始的路径数组
///
/// # 示例
/// ```rust
/// use math_utils::generate_new_path_array;
///
/// let path = vec![(0.0, 0.0), (10.0, 10.0), (20.0, 20.0), (30.0, 30.0)];
/// let current = (15.0, 15.0);
/// let new_path = generate_new_path_array(&path, current);
/// // new_path 将从 (20.0, 20.0) 开始，因为它是距离 (15.0, 15.0) 最近的点
/// ```
/// todo: 待优化，较python版慢
pub fn generate_new_path_array(path_array: &[(f64, f64)], current_coord: (f64, f64)) -> Vec<(f64, f64)> {
    if path_array.is_empty() {
        return Vec::new();
    }

    let (current_x, current_y) = current_coord;
    let mut min_distance = f64::INFINITY;
    let mut min_distance_index = 0;

    // 遍历路径数组中的每个点，计算距离
    for (index, point) in path_array.iter().enumerate() {
        let distance = calculate_distance(current_x, current_y, point.0, point.1);
        // 如果找到更短的距离，更新最短距离和索引
        if distance < min_distance {
            min_distance = distance;
            min_distance_index = index;
        }
    }

    // 从最短距离的点开始构建新的路径数组
    path_array[min_distance_index..].to_vec()
}

/// 根据当前坐标和路径数组，生成一个新的路径数组，从距离当前坐标最近的点开始（整数坐标版本）
///
/// # 参数
/// - `path_array`: 路径数组，包含多个坐标点
/// - `current_coord`: 当前坐标 (x, y)
///
/// # 返回
/// 从距离当前坐标最近的点开始的路径数组
///
/// # 示例
/// ```rust
/// use math_utils::generate_new_path_array_i32;
///
/// let path = vec![(0, 0), (10, 10), (20, 20), (30, 30)];
/// let current = (15, 15);
/// let new_path = generate_new_path_array_i32(&path, current);
/// // new_path 将从 (20, 20) 开始，因为它是距离 (15, 15) 最近的点
/// ```
pub fn generate_new_path_array_i32(path_array: &[(i32, i32)], current_coord: (i32, i32)) -> Vec<(i32, i32)> {
    if path_array.is_empty() {
        return Vec::new();
    }

    let (current_x, current_y) = current_coord;
    let mut min_distance = f64::INFINITY;
    let mut min_distance_index = 0;

    // 遍历路径数组中的每个点，计算距离
    for (index, point) in path_array.iter().enumerate() {
        let distance = calculate_distance_i32(current_x, current_y, point.0, point.1);
        // 如果找到更短的距离，更新最短距离和索引
        if distance < min_distance {
            min_distance = distance;
            min_distance_index = index;
        }
    }

    // 从最短距离的点开始构建新的路径数组
    path_array[min_distance_index..].to_vec()
}

/// 计算_求斜率旧未修改版（根据两点运算角度）
///
/// 根据两点坐标计算角度，返回 0-360 度的角度值
/// 角度以 y 轴正方向为 0 度，逆时针递增
///
/// # 参数
/// - `x1`: 第一个点的 X 坐标
/// - `y1`: 第一个点的 Y 坐标
/// - `x2`: 第二个点的 X 坐标
/// - `y2`: 第二个点的 Y 坐标
///
/// # 返回
/// 角度值（0-360 度），如果两点相同或无法计算则返回 NaN
///
/// # 示例
/// ```rust
/// use math_utils::calculate_angle_old;
///
/// // 计算从 (0, 0) 到 (1, 0) 的角度（应该是 90 度）
/// let angle = calculate_angle_old(0.0, 0.0, 1.0, 0.0);
/// assert!((angle - 90.0).abs() < 1e-10);
/// ```
pub fn calculate_angle_old(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    
    // 处理 x1 和 x2 相等的情况
    if dx == 0.0 {
        if dy < 0.0 {
            return 0.0;
        } else if dy > 0.0 {
            return 180.0;
        } else {
            // 两点相同，返回 NaN
            return f64::NAN;
        }
    }
    
    // 处理 y1 和 y2 相等的情况
    if dy == 0.0 {
        if dx > 0.0 {
            return 90.0;
        } else if dx < 0.0 {
            return 270.0;
        }
    }
    
    // 计算反正切值并转换为角度
    let mut angle = (dy / dx).atan() / PI * 180.0;
    
    // 根据象限调整角度
    if dy < 0.0 && dx > 0.0 {
        angle += 90.0;
    } else if dy > 0.0 && dx > 0.0 {
        angle += 90.0;
    } else if dy > 0.0 && dx < 0.0 {
        angle += 270.0;
    } else if dy < 0.0 && dx < 0.0 {
        angle += 270.0;
    }
    
    angle
}

/// 计算_求转角度旧（计算需要转动的角度和方向）
///
/// 根据当前朝向角度和朝向坐标、目的地坐标，计算需要转动的角度和方向
///
/// # 参数
/// - `current_angle`: 当前朝向角度（0-360 度）
/// - `current_x`: 当前朝向坐标 X
/// - `current_y`: 当前朝向坐标 Y
/// - `target_x`: 目的地坐标 X
/// - `target_y`: 目的地坐标 Y
///
/// # 返回
/// 元组 (方向, 角度)，其中：
/// - 方向：`RotationDirection::Left` 或 `RotationDirection::Right`
/// - 角度：需要转动的角度（0-180 度）
///
/// # 示例
/// ```rust
/// use math_utils::{calculate_rotation_angle_old, RotationDirection};
///
/// // 当前朝向 90 度，朝向坐标 (0, 0)，目的地 (1, 1)
/// let (direction, angle) = calculate_rotation_angle_old(90.0, 0.0, 0.0, 1.0, 1.0);
/// // 应该需要向左或向右转动一定角度
/// ```
pub fn calculate_rotation_angle_old(
    current_angle: f64,
    current_x: f64,
    current_y: f64,
    target_x: f64,
    target_y: f64,
) -> (RotationDirection, f64) {
    // 计算从朝向坐标到目的地的角度
    let target_angle = calculate_angle_old(current_x, current_y, target_x, target_y);
    
    // 如果角度计算失败（NaN），返回默认值
    if target_angle.is_nan() {
        return (RotationDirection::Right, 0.0);
    }
    
    // 计算角度差
    let mut angle_diff = target_angle - current_angle;
    
    // 标准化角度差到 [-360, 360] 范围
    while angle_diff > 360.0 {
        angle_diff -= 360.0;
    }
    while angle_diff < -360.0 {
        angle_diff += 360.0;
    }
    
    // 根据角度差判断方向和计算转动角度
    if angle_diff > 0.0 && angle_diff < 180.0 {
        // 角度差在 (0, 180) 之间，向右转
        (RotationDirection::Right, angle_diff)
    } else if angle_diff >= -180.0 && angle_diff < 0.0 {
        // 角度差在 [-180, 0) 之间，向左转
        (RotationDirection::Left, angle_diff.abs())
    } else if angle_diff >= 180.0 && angle_diff < 360.0 {
        // 角度差在 [180, 360) 之间，向左转（更短路径）
        (RotationDirection::Left, 360.0 - angle_diff)
    } else if angle_diff >= -360.0 && angle_diff < -180.0 {
        // 角度差在 [-360, -180) 之间，向右转（更短路径）
        (RotationDirection::Right, 360.0 + angle_diff)
    } else {
        // 其他情况（包括 angle_diff == 0），默认向右转，角度为 0
        (RotationDirection::Right, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        // 测试 3-4-5 直角三角形
        let distance = calculate_distance(0.0, 0.0, 3.0, 4.0);
        assert!((distance - 5.0).abs() < 1e-10);
        
        // 测试相同点
        let distance = calculate_distance(5.0, 5.0, 5.0, 5.0);
        assert!((distance - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_distance_i32() {
        // 测试 3-4-5 直角三角形
        let distance = calculate_distance_i32(0, 0, 3, 4);
        assert!((distance - 5.0).abs() < 1e-10);
        
        // 测试相同点
        let distance = calculate_distance_i32(5, 5, 5, 5);
        assert!((distance - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_generate_new_path_array() {
        // 测试基本功能：当前坐标 (12.0, 12.0) 距离 (10.0, 10.0) 最近
        let path = vec![(0.0, 0.0), (10.0, 10.0), (20.0, 20.0), (30.0, 30.0)];
        let current = (12.0, 12.0);
        let new_path = generate_new_path_array(&path, current);
        // (10.0, 10.0) 是距离 (12.0, 12.0) 最近的点
        assert_eq!(new_path.len(), 3);
        assert_eq!(new_path[0], (10.0, 10.0));
        assert_eq!(new_path[1], (20.0, 20.0));
        assert_eq!(new_path[2], (30.0, 30.0));

        // 测试当前坐标距离最后一个点最近
        let path = vec![(0.0, 0.0), (10.0, 10.0), (20.0, 20.0)];
        let current = (25.0, 25.0);
        let new_path = generate_new_path_array(&path, current);
        assert_eq!(new_path.len(), 1);
        assert_eq!(new_path[0], (20.0, 20.0));

        // 测试空数组
        let empty_path: Vec<(f64, f64)> = Vec::new();
        let new_path = generate_new_path_array(&empty_path, (0.0, 0.0));
        assert_eq!(new_path.len(), 0);

        // 测试当前坐标就是路径中的第一个点
        let path = vec![(0.0, 0.0), (10.0, 10.0), (20.0, 20.0)];
        let current = (0.0, 0.0);
        let new_path = generate_new_path_array(&path, current);
        assert_eq!(new_path.len(), 3);
        assert_eq!(new_path[0], (0.0, 0.0));
    }

    #[test]
    fn test_generate_new_path_array_i32() {
        // 测试基本功能：当前坐标 (12, 12) 距离 (10, 10) 最近
        let path = vec![(0, 0), (10, 10), (20, 20), (30, 30)];
        let current = (12, 12);
        let new_path = generate_new_path_array_i32(&path, current);
        // (10, 10) 是距离 (12, 12) 最近的点
        assert_eq!(new_path.len(), 3);
        assert_eq!(new_path[0], (10, 10));
        assert_eq!(new_path[1], (20, 20));
        assert_eq!(new_path[2], (30, 30));

        // 测试当前坐标距离最后一个点最近
        let path = vec![(0, 0), (10, 10), (20, 20)];
        let current = (25, 25);
        let new_path = generate_new_path_array_i32(&path, current);
        assert_eq!(new_path.len(), 1);
        assert_eq!(new_path[0], (20, 20));

        // 测试空数组
        let empty_path: Vec<(i32, i32)> = Vec::new();
        let new_path = generate_new_path_array_i32(&empty_path, (0, 0));
        assert_eq!(new_path.len(), 0);

        // 测试当前坐标就是路径中的第一个点
        let path = vec![(0, 0), (10, 10), (20, 20)];
        let current = (0, 0);
        let new_path = generate_new_path_array_i32(&path, current);
        assert_eq!(new_path.len(), 3);
        assert_eq!(new_path[0], (0, 0));
    }

    #[test]
    fn test_calculate_angle_old() {
        use super::*;
        
        // 测试从 (0, 0) 到 (1, 0) 的角度（应该是 90 度）
        let angle = calculate_angle_old(0.0, 0.0, 1.0, 0.0);
        assert!((angle - 90.0).abs() < 1e-10);
        
        // 测试从 (0, 0) 到 (0, 1) 的角度（应该是 180 度）
        let angle = calculate_angle_old(0.0, 0.0, 0.0, 1.0);
        assert!((angle - 180.0).abs() < 1e-10);
        
        // 测试从 (0, 0) 到 (-1, 0) 的角度（应该是 270 度）
        let angle = calculate_angle_old(0.0, 0.0, -1.0, 0.0);
        assert!((angle - 270.0).abs() < 1e-10);
        
        // 测试从 (0, 0) 到 (0, -1) 的角度（应该是 0 度）
        let angle = calculate_angle_old(0.0, 0.0, 0.0, -1.0);
        assert!((angle - 0.0).abs() < 1e-10);
        
        // 测试相同点（应该返回 NaN）
        let angle = calculate_angle_old(0.0, 0.0, 0.0, 0.0);
        assert!(angle.is_nan());
    }

    #[test]
    fn test_calculate_rotation_angle_old() {
        use super::*;
        
        // 测试：当前朝向 90 度，朝向坐标 (0, 0)，目的地 (1, 0)
        // 目标角度应该是 90 度，角度差为 0，应该返回 (Right, 0.0)
        let (direction, angle) = calculate_rotation_angle_old(90.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(direction, RotationDirection::Right);
        assert!((angle - 0.0).abs() < 1e-10);
        
        // 测试：当前朝向 0 度，朝向坐标 (0, 0)，目的地 (1, 0)
        // 目标角度应该是 90 度，角度差为 90，应该向右转 90 度
        let (direction, angle) = calculate_rotation_angle_old(0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(direction, RotationDirection::Right);
        assert!((angle - 90.0).abs() < 1e-10);
        
        // 测试：当前朝向 90 度，朝向坐标 (0, 0)，目的地 (0, 1)
        // 目标角度应该是 180 度，角度差为 90，应该向右转 90 度
        let (direction, angle) = calculate_rotation_angle_old(90.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(direction, RotationDirection::Right);
        assert!((angle - 90.0).abs() < 1e-10);
        
        // 测试：当前朝向 180 度，朝向坐标 (0, 0)，目的地 (1, 0)
        // 目标角度应该是 90 度，角度差为 -90，应该向左转 90 度
        let (direction, angle) = calculate_rotation_angle_old(180.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(direction, RotationDirection::Left);
        assert!((angle - 90.0).abs() < 1e-10);
        
        // 测试：当前朝向 90 度，朝向坐标 (0, 0)，目的地 (-1, 0)
        // 目标角度应该是 270 度，角度差为 180，应该向左转 180 度（更短路径）
        let (direction, angle) = calculate_rotation_angle_old(90.0, 0.0, 0.0, -1.0, 0.0);
        assert_eq!(direction, RotationDirection::Left);
        assert!((angle - 180.0).abs() < 1e-10);
    }
}
