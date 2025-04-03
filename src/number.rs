use std::mem::transmute;

/// Represents supported number types for reinterpretation.
#[derive(Debug)]
pub enum Number {
    I32(i32),
    U32(u32),
    F32(f32),
    I64(i64),
    U64(u64),
    F64(f64),
}

impl Number {
     /// Reinterprets the number as a 32-bit signed integer (`i32`) if valid.
    ///
    /// Supported conversions:
    /// - `U32` to `i32` using `transmute`
    /// - `F32` to `i32` via bit conversion
    ///
    /// Returns `None` if the conversion is invalid or from a 64-bit number.
    pub fn reinterpret_as_i32(&self) -> Option<i32> {
        match self {
            Number::U32(n) => Some(unsafe { transmute(*n) }),
            Number::F32(n) => Some(n.to_bits() as i32),
            _ => None,
        }
    }
    /// Reinterprets the number as a 32-bit floating point (`f32`) if valid.
    ///
    /// Supported conversions:
    /// - `I32` to `f32` using `from_bits`
    /// - `U32` to `f32` using `from_bits`
    ///
    /// Returns `None` if the conversion is invalid or from a 64-bit number.
    pub fn reinterpret_as_f32(&self) -> Option<f32> {
        match self {
            Number::I32(n) => Some(f32::from_bits(*n as u32)),
            Number::U32(n) => Some(f32::from_bits(*n)),
            _ => None,
        }
    }
}
