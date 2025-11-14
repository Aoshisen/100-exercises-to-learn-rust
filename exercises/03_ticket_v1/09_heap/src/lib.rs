pub struct Ticket {
    title: String,
    description: String,
    status: String,
}
// pub struct String {
//     ptr: *mut u8, // 指向实际字符串数据的指针 (8 bytes)#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        // String 包含 ptr, len, cap 三个 usize 大小的字段
        assert_eq!(size_of::<String>(), size_of::<usize>() * 3);
    }

    #[test]
    fn ticket_size() {
        // Ticket 包含三个 String 字段
        assert_eq!(size_of::<Ticket>(), size_of::<String>() * 3);
    }
    
    #[test]
    fn verify_pointer_size() {
        // 验证指针大小（在 64 位系统上应该是 8）
        assert_eq!(size_of::<*mut u8>(), size_of::<usize>());
    }
} 
//     len: usize,   // 字符串的实际长度 (8 bytes)
//     cap: usize,   // 分配的内存容量 (8 bytes)
// }
