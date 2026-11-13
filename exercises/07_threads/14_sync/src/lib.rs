// Not much to be exercised on `Sync`, just a thing to remember.
fn outro() -> &'static str {
    "I have a good understanding of Send and Sync!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a good understanding of Send and Sync!");
    }
}

// 用两个例子把直觉钉死
// 例 1：RefCell<T> —— 能搬，不能借
// rust
// // RefCell 的借用计数器是普通的 Cell<usize>，不是原子的
// let cell = RefCell::new(0);

// // ❌ 不能借给两个线程：两个线程同时 borrow_mut 会踩爆计数器
// // （这就是为什么 RefCell: !Sync）

// // ✅ 但可以整个搬走：搬走后只有一个线程能碰它，没有并发
// thread::spawn(move || {
//     *cell.borrow_mut() = 1;  // 这里独占，安全
// });
// 搬走 (Send)：RefCell<T>: Send 当 T: Send。因为所有权转移后，只有一个线程访问它，借用计数器的并发问题不存在。

// 借出去 (!Sync)：&RefCell 一旦跨线程共享，两个线程就能同时 borrow_mut() 拿到 &mut，数据竞争 → UB。

// 一句话：RefCell 怕的是"同时被两个人借"，不怕"被一个人搬走"。

// 例 2：MutexGuard —— 能借，不能搬
// rust
// let lock = Mutex::new(0);
// let guard = lock.lock().unwrap();

// // ❌ 不能搬到别的线程：
// //    drop(guard) 会在新线程里执行，但锁必须在获取它的线程释放！
// // （这就是为什么 MutexGuard: !Send）

// // ✅ 但可以借给别人看一眼：
// thread::scope(|s| {
//     s.spawn(|| {
//         let _: &MutexGuard<i32> = &guard;  // 只是读一下，没影响
//     });
// });
// 搬走 (!Send)：锁的释放必须发生在加锁的同一个线程。如果把 MutexGuard 移到别的线程，它会在错误的线程 drop，破坏 Mutex 的契约。

// 借出去 (Sync)：把 &MutexGuard 给另一个线程，对方没有所有权，没法 drop，锁依然在原线程释放。而且 Deref 只给出 &T（当 T: Sync 时安全）。所以共享 &MutexGuard 没问题。

// 一句话：MutexGuard 怕的是"在别人的地盘上被销毁"，不怕"被别人瞅一眼"。

// 一张表总结
// 类型	Send?	Sync?	原因
// i32, String	✅	✅	没问题
// Rc<T>	❌	❌	引用计数非原子，搬走/共享都危险
// Arc<T>	✅ (T: Send+Sync)	✅ (T: Send+Sync)	原子引用计数
// RefCell<T>	✅ (T: Send)	❌	能独占搬走，不能并发共享
// MutexGuard<T>	❌	✅ (T: Sync)	能借看，不能在别的线程释放
// Cell<T>	✅	❌	同 RefCell 的道理
