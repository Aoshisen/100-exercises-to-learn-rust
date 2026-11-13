pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24);
    }
    // pub struct NonNull<T: PointeeSized> {
    //     pointer: *const T,   // 8 字节
    // }
    // RawVecInner {
    //     ptr: Unique<u8>,   // NonNull<u8> → u8*，8 字节（非零 niche）
    //     cap: Cap,          // UsizeNoHighBit(usize)，8 字节（最高位必须是 0，也是 niche）
    //     alloc: A,          // Global，ZST，0 字节
    // }
    // RawVec {
    //     inner: RawVecInner,   // 16 字节
    //     _marker: PhantomData<T>, // ZST，0 字节
    // }
    // Vec {
    //     buf: RawVec,  // 16 字节
    //     len: usize,   // 8 字节
    // }
    // String {
    //     vec: Vec<u8>, // 24 字节
    // }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
