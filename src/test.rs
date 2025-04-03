#[cfg(test)]
mod tests {
    use super::number::Number;
    
    #[test]
    fn test_reinterpret_as_i32() {
        let num = Number::U32(1065353216); // 1.0f32 as u32
        assert_eq!(num.reinterpret_as_i32(), Some(1065353216 as i32));
    }
    
    #[test]
    fn test_reinterpret_as_f32() {
        let num = Number::I32(1065353216); // 1.0f32 as i32
        assert_eq!(num.reinterpret_as_f32(), Some(1.0));
    }
}
