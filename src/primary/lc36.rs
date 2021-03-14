/// 有效的数独
/// 判断一个 9x9 的数独是否有效。只需要根据以下规则，验证已经填入的数字是否有效即可。
/// 数字 1-9 在每一行只能出现一次。
///
/// 数字 1-9 在每一列只能出现一次。
///
/// 数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut res:Vec<Vec<char>> = Vec::with_capacity(27);
    // 9 行(0 ~ 8) 9 列 (9 ~ 17 ) 9 方块(18 ~ 26)
    res.resize(27, vec![]);
    for (i, row) in board.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            // 计算该位置的数所处位置 一个数一定存在于3个数组中
            res[i].push(c.to_owned().clone());
            res[9 + j].push(c.to_owned().clone());
            res[18 + i / 3 * 3 + j / 3].push(c.to_owned().clone());
        }
    }
    res.iter().all(| a| is_valid(a))
}

#[inline]
fn is_valid(group: &Vec<char>) -> bool {
    // 9 个数是否有效
    let mut v = Vec::with_capacity(9);
    v.resize(9, 0);
    for c in group {
        if let Ok(n) =  c.to_string().parse::<usize>() {
            if n > 9 || n < 1 {
                return false
            } else {
                v[n - 1] = v[n - 1] + 1;
            }
        }
    }
    v.iter().all(|a| *a < 2)
}

#[test]
fn test_is_valid_sudoku() {
    let s = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9']
    ];
    assert_eq!(is_valid_sudoku(s), true)
}

#[test]
fn test_is_valid() {
    let v = vec!['.','9','8','.','.','.','.','6','.'];
    assert_eq!(is_valid(&v), true);
    let v = vec!['.','9','0','.','.','.','.','6','.'];
    assert_eq!(is_valid(&v), false);
    let v = vec!['9','9','8','.','.','.','.','6','.'];
    assert_eq!(is_valid(&v), false);
}
