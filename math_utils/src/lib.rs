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
}
