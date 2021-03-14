/// 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
/// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？

pub fn climb_stairs(n: i32) -> i32 {
    if n < 3 {
        n
    } else {
        let mut v = Vec::with_capacity(n as usize);
        for i in 1..=n  {
            if i < 3 {
                v.push(i);
            } else {
                let len = v.len();
                v.push(v[len - 1] + v[len - 2])
            }
        }
        *v.last().unwrap()
    }
}

#[test]
fn test_climb_stairs() {
    assert_eq!(climb_stairs(5), 8);
}