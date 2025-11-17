// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // 泄漏堆分配，获取静态切片
    let static_slice: &'static mut [i32] = v.leak();

    // 处理空切片的情况
    if static_slice.is_empty() {
        return 0;
    }

    // 计算中间位置，确保正确分割成两半
    let mid = static_slice.len() / 2;

    // 使用 split_at 正确分割成两半
    let (left, right) = static_slice.split_at(mid);

    [left, right]
        .into_iter()
        .map(|slice| thread::spawn(move || slice.iter().sum::<i32>()))
        .map(|handle| handle.join().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
