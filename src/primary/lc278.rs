// The API isBadVersion is defined for you.
// isBadVersion(versions:i32)-> bool;
// to call it use self.isBadVersion(versions)
/// 你是产品经理，目前正在带领一个团队开发新的产品。不幸的是，你的产品的最新版本没有通过质量检测。由于每个版本都是基于之前的版本开发的，所以错误的版本之后的所有版本都是错的。
/// 返回第一个错误的版本
pub fn first_bad_version(n: i32) -> i32 {
    let (mut start, mut end)  = (1, n);
    while start < end {
        let mid = start + (end - start) / 2;
        if is_bad_version(mid) {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    end
}

fn is_bad_version(version: i32) -> bool {
     version >= 3
}

#[test]
fn test_first_bad_version() {
    assert_eq!(first_bad_version(7), 3);
}
