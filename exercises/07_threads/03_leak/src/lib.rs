// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let half_len = v.len() / 2;
    // v.leak() 返回 &'static [i32]，不再受限于当前函数作用域
    let (head, tail) = v.leak().split_at(half_len);

    // 因为 head 和 tail 是 &'static，标准 spawn 可以自由地将其 move 进新线程
    let head_handle = thread::spawn(move || head.iter().sum::<i32>());
    let tail_handle = thread::spawn(move || tail.iter().sum::<i32>());

    head_handle.join().unwrap() + tail_handle.join().unwrap()
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
