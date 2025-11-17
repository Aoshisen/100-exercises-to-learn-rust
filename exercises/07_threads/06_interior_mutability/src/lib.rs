// TODO: Use `Rc` and `RefCell` to implement `DropTracker<T>`, a wrapper around a value of type `T`
//  that increments a shared `usize` counter every time the wrapped value is dropped.

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub struct DropTracker<T> {
    value: T,
    counter: Rc<RefCell<u32>>,
}

impl<T> DropTracker<T> {
    pub fn new(value: T, counter: Rc<RefCell<u32>>) -> Self {
        Self { value, counter }
    }
}

impl<T> Drop for DropTracker<T> {
    fn drop(&mut self) {
        //怎么去改变borrow
        *self.counter.borrow_mut() += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let counter = Rc::new(RefCell::new(0));
        let _ = DropTracker::new((), Rc::clone(&counter));
        // 1. 创建 DropTracker 实例
        // 2. Rc::clone(&counter) 使 counter 引用计数变为 2
        // 3. DropTracker 实例立即离开作用域 (因为使用 _)
        // 4. 自动调用 drop 方法:
        //    *self.counter.borrow_mut() += 1;  // 计数器增加到 1
        // 5. DropTracker 被销毁，内部的 counter clone 也被销毁
        // 6. counter 引用计数回到 1
        assert_eq!(*counter.borrow(), 1);
    }

    #[test]
    fn multiple() {
        let counter = Rc::new(RefCell::new(0));

        {
            let a = DropTracker::new(5, Rc::clone(&counter));
            let b = DropTracker::new(6, Rc::clone(&counter));
        }

        // 第一步：通过 Deref trait 从 Rc<RefCell<u32>> 获取 &RefCell<u32>
        // 这是自动发生的，因为 Rc 实现了 Deref trait

        // 第二步：调用 RefCell 的 borrow() 方法
        // counter.borrow()  // 返回 Ref<u32> 类型
        assert_eq!(*counter.borrow(), 2);
    }
}
