
/// 给定一个 n × n 的二维矩阵 matrix 表示一个图像。请你将图像顺时针旋转 90 度。
///
/// 你必须在 原地 旋转图像，这意味着你需要直接修改输入的二维矩阵。请不要 使用另一个矩阵来旋转图像。
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    // 次对角线翻转
    for i in 0..(n - 1) {
        for j in 0..(n - 1 - i) {
            let (a, b) = (matrix[i][j], matrix[n - 1 - j][n - 1 - i]);
            matrix[i][j] = b;
            matrix[n - 1 - j][n - 1 - i] = a ;
        }
    }
    // 上下翻转
    for i in 0..(n / 2) {
        matrix.swap(i, n - 1 - i);
    }
}

#[test]
fn test_rotate() {
    let mut m = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    rotate(&mut m);
    assert_eq!(m,vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]]);

}