pub mod i32;

#[inline(always)]
pub fn sb_mod(a: f64, b: f64) -> f64 {
    let t = a % b;
    if t < 0f64 {
        t + b
    } else {
        t
    }
}

#[cfg(test)]
mod tests {
    use crate::sb_mod;

    #[test]
    fn test_sb_mod() {
        assert_eq!(sb_mod(10f64, 3f64), 1f64);
        assert_eq!(sb_mod(-100f64, 9f64), 8f64);
        assert_eq!(sb_mod(3f64, 99f64), 3f64);
        assert_eq!(sb_mod(999f64, 1f64), 0f64);
        assert!(sb_mod(999f64, 0f64).is_nan());
    }
}
