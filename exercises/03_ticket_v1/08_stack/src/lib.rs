// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn u16_size() {
        //一个16位无符号整数占2字节
        assert_eq!(size_of::<u16>(), 2);
    }

    #[test]
    fn i32_size() {
        //一个32位有符号整数占4字节
        assert_eq!(size_of::<i32>(), 4);
    }

    #[test]
    fn bool_size() {
        //一个布尔占1字节
        assert_eq!(size_of::<bool>(), 1);
    }
}
