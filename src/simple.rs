/// 删除排序数组中的重复项
/// 给定一个排序数组，你需要在原地删除重复出现的元素，使得每个元素只出现一次，返回移除后数组的新长度
/// 不要使用额外的数组空间，你必须在原地修改输入数组 并在使用 O(1) 额外空间的条件下完成。
///
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut p1 = 0;
    let mut p2 = 1;
    let mut i = 0;
    while p2 < nums.len() {
        if nums[p2] == nums[p1] {
            p2 += 1;
        } else {
            p1 += 1;
            nums[p1] = nums[p2];
            i += 1;
        }
    }
    i + 1
}
/// 给定一个字符串，找到它的第一个不重复的字符，并返回它的索引。如果不存在，则返回 -1
pub fn first_uniq_char(s: String) -> i32 {
    let mut i = -1;
    let mut m = std::collections::HashMap::new();
    for c in s.chars() {
        if m.contains_key(&c) {
            let v = m.get_mut(&c).unwrap();
            *v += 1;
        } else {
            m.insert(c, 1);
        }
    }
    for (j, c) in s.chars().enumerate() {
        if *m.get(&c).unwrap() == 1 {
            i = j as i32;
            break;
        }
    }
    i
}

///给定一个字符串，验证它是否是回文串，只考虑字母和数字字符，可以忽略字母的大小写。
pub fn is_palindrome(s: String) -> bool {
    if s.len() <= 1 {
        true
    } else {
        let (mut p1, mut p2) = (0, s.len() - 1);
        let v: Vec<char> = s.chars().collect();
        while p2 > p1 {
            // 检验有效
            if !v[p1].is_ascii_alphanumeric() {
                p1 += 1;
                continue;
            }
            if !v[p2].is_ascii_alphanumeric() {
                p2 -= 1;
                continue;
            }
            if v[p1].to_ascii_lowercase() == v[p2].to_ascii_lowercase() {
                p1 += 1;
                p2 -= 1;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_duplicates() {
        let mut v = vec![1, 1, 2, 3, 4, 4, 5];
        let res = remove_duplicates(&mut v);
        assert_eq!(res, 5);
    }
    #[test]
    fn test_first_uniq_char() {
        let s = "leetcode";
        assert_eq!(first_uniq_char(s.into()), 0)
    }
    #[test]
    fn test_is_palindrome() {
        let s = "a.";
        assert_eq!(is_palindrome(s.into()), true);
    }
}
