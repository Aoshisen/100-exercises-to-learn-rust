// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let half_len = v.len() / 2;
    let mut head = v;
    let tail = head.split_off(half_len);
    let head_handle = thread::spawn(move || head.iter().sum::<i32>());
    let tail_handle = thread::spawn(move || tail.iter().sum::<i32>());
    head_handle.join().unwrap() + tail_handle.join().unwrap()
}
//这里实现有三种方案
// 1. 使用作用域线程 std::thread::scope这个scope 里面的线程可以安全的借用局部变量
// use std::thread;

// pub fn sum(v: Vec<i32>) -> i32 {
//     let half_length = v.len() / 2;
//     let mut v = v;
//     let (head, tail) = v.split_at_mut(half_length);

//     // 创建一个作用域，该作用域内的线程可以安全地借用局部变量 head 和 tail
//     thread::scope(|s| {
//         let head_handle = s.spawn(|| head.iter().sum::<i32>());
//         let tail_handle = s.spawn(|| tail.iter().sum::<i32>());

//         // 两个线程会在作用域结束前自动被 join
//         head_handle.join().unwrap() + tail_handle.join().unwrap()
//     })
// }

//2. 使用rayon （最佳实践）
// 需要在 Cargo.toml 中添加 rayon = "1.10"
// use rayon::prelude::*;

// pub fn sum(v: Vec<i32>) -> i32 {
//     // par_iter() 会自动在后台将底层 slice 拆分给 Rayon 的线程池处理
//     v.par_iter().sum::<i32>()
// }

//3. 必须要转移所有权

// use std::thread;

// pub fn sum(v: Vec<i32>) -> i32 {
//     let half_length = v.len() / 2;
//     let mut head = v;
//     // split_off 会截断原 Vec，返回一个包含后半部分数据所有权的新 Vec
//     let tail = head.split_off(half_length);

//     // 此时 head 和 tail 都是拥有独立所有权的 Vec，满足 'static 约束
//     let head_handle = thread::spawn(move || head.iter().sum::<i32>());
//     let tail_handle = thread::spawn(move || tail.iter().sum::<i32>());

//     head_handle.join().unwrap() + tail_handle.join().unwrap()
// }

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
