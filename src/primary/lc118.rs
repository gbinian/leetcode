/// 杨辉三角
/// 给定一个非负整数 numRows，生成杨辉三角的前 numRows 行
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut n = 1_usize;
    while n <= num_rows as usize {
        if n == 1 {
            res.push(vec![1]);
        } else {
            let mut row:Vec<i32> = Vec::with_capacity(n);
            let prev_row = &res[n - 2];
            for i in 0..n  {
                if i == 0 || i == n - 1 { row.push(1); }
                else {row.push(prev_row[i - 1] + prev_row[i]); }
            }
            res.push(row);
        }
        n += 1;
    }
    res
}

#[test]
fn test_generate() {
    assert_eq!(generate(4), vec![
          vec![1],
         vec![1,1],
        vec![1,2,1],
       vec![1,3,3,1]
    ])
}