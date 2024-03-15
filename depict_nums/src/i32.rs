use crate::sb_mod;

#[derive(Debug, PartialEq, Clone, Copy)]
struct DepictI32(f64);

impl From<DepictI32> for i32 {
    fn from(x: DepictI32) -> i32 {
        x.0 as i32
    }
}

impl From<i32> for DepictI32 {
    fn from(x: i32) -> DepictI32 {
        DepictI32(x as f64)
    }
}

impl std::ops::Add for DepictI32 {
    type Output = DepictI32;

    fn add(self, rhs: DepictI32) -> Self::Output {
        // let result = self.0 + rhs.0;
        // if result > std::i32::MAX as f64 {
        //     // Overflow occurred, wrap around
        //     DepictI32((result - std::i32::MAX as f64) + std::i32::MIN as f64 - 1.0)
        // } else if result < std::i32::MIN as f64 {
        //     // Underflow occurred, wrap around
        //     DepictI32((result - std::i32::MIN as f64) + std::i32::MAX as f64 + 1.0)
        // } else {
        //     DepictI32(result)
        // }
        DepictI32(
            sb_mod(
                self.0 + rhs.0 + (std::i32::MAX as f64 + 1.0),
                (std::i32::MAX as f64 + 1.0) * 2.0,
            ) - (std::i32::MAX as f64 + 1.0),
        )
    }
}

impl std::ops::Sub for DepictI32 {
    type Output = DepictI32;

    fn sub(self, rhs: DepictI32) -> Self::Output {
        // let result = self.0 - rhs.0;
        // if result > std::i32::MAX as f64 {
        //     // Overflow occurred, wrap around
        //     DepictI32((result - std::i32::MAX as f64) + std::i32::MIN as f64 - 1.0)
        // } else if result < std::i32::MIN as f64 {
        //     // Underflow occurred, wrap around
        //     DepictI32((result - std::i32::MIN as f64) + std::i32::MAX as f64 + 1.0)
        // } else {
        //     DepictI32(result)
        // }
        DepictI32(
            sb_mod(
                self.0 - rhs.0 + (std::i32::MAX as f64 + 1.0),
                (std::i32::MAX as f64 + 1.0) * 2.0,
            ) - (std::i32::MAX as f64 + 1.0),
        )
    }
}

impl std::ops::Mul for DepictI32 {
    type Output = DepictI32;

    fn mul(self, rhs: DepictI32) -> Self::Output {
        // println!("\n====================");

        // println!("self: {}, rhs: {}", self.0, rhs.0);

        // 上の桁と下の桁を分けて計算する
        let b = sb_mod(self.0, (1 << 16) as f64);
        let a = (self.0 - b) / (1 << 16) as f64;
        let d = sb_mod(rhs.0, (1 << 16) as f64);
        let c = (rhs.0 - d) / (1 << 16) as f64;

        // self = a * 2^16 + b
        // rhs  = c * 2^16 + d

        assert_eq!(self.0, a * (1 << 16) as f64 + b);
        assert_eq!(rhs.0, c * (1 << 16) as f64 + d);

        // println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);

        // let ac = a * c;
        let ad = a * d;
        let bc = b * c;
        let bd = b * d;

        // println!("ac: {}, ad: {}, bc: {}, bd: {}", ac, ad, bc, bd);

        // acは2^32からの位までの桁なのでオーバーフローして0になる
        // adとbcは2^16からの位までの桁なので2^16より大きい桁はオーバーフロー
        // bdは2^0からの位までの桁なのでオーバーフローしない
        let ad_down = sb_mod(ad, (1 << 16) as f64);
        let bc_down = sb_mod(bc, (1 << 16) as f64);
        let result = (ad_down + bc_down) * (1 << 16) as f64 + bd;

        // println!("result: {}", result);

        let result = sb_mod(
            result + (std::i32::MAX as f64 + 1.0),
            (std::i32::MAX as f64 + 1.0) * 2.0,
        ) - (std::i32::MAX as f64 + 1.0);

        // println!(
        //     "ad_down: {}, bc_down: {}, result: {}",
        //     ad_down, bc_down, result
        // );

        DepictI32(result)
    }
}

#[cfg(test)]
mod tests {
    mod add {
        use crate::i32::DepictI32;

        #[test]
        fn test_addition_without_overflow_plus_plus() {
            let a: DepictI32 = 10.into();
            let b: DepictI32 = 20.into();
            let result: DepictI32 = a + b;
            assert_eq!(Into::<i32>::into(result), 30);
        }

        #[test]
        fn test_addition_without_overflow_minus_plus() {
            let a: DepictI32 = (-10).into();
            let b: DepictI32 = 20.into();
            let result: DepictI32 = a + b;
            assert_eq!(Into::<i32>::into(result), 10);
        }

        #[test]
        fn test_addition_without_overflow_plus_minus() {
            let a: DepictI32 = 10.into();
            let b: DepictI32 = (-20).into();
            let result: DepictI32 = a + b;
            assert_eq!(Into::<i32>::into(result), -10);
        }

        #[test]
        fn test_addition_without_overflow_minus_minus() {
            let a: DepictI32 = (-10).into();
            let b: DepictI32 = 20.into();
            let result: DepictI32 = a + b;
            assert_eq!(Into::<i32>::into(result), 10);
        }

        #[test]
        fn test_addition_with_overflow_plus_plus() {
            for diff_1 in -10..10 {
                for diff_2 in -10..10 {
                    let _a = std::i32::MAX.wrapping_add(diff_1);
                    let _b = std::i32::MAX.wrapping_add(diff_2);
                    let a: DepictI32 = _a.into();
                    let b: DepictI32 = _b.into();
                    let result = a + b;
                    assert_eq!(Into::<i32>::into(result), _a.wrapping_add(_b),);
                }
            }
        }

        #[test]
        fn test_addition_with_underflow_minus_minus() {
            for diff_1 in -10..10 {
                for diff_2 in -10..10 {
                    let _a = std::i32::MIN.wrapping_add(diff_1);
                    let _b = std::i32::MIN.wrapping_add(diff_2);
                    let a: DepictI32 = _a.into();
                    let b: DepictI32 = _b.into();
                    let result = a + b;
                    assert_eq!(Into::<i32>::into(result), _a.wrapping_add(_b),);
                }
            }
        }

        use rand::distributions::Uniform;
        use rand::Rng;

        #[test]
        fn test_addition_consistency_with_i32() {
            let mut rng = rand::thread_rng();
            let range = Uniform::new_inclusive(std::i32::MIN, std::i32::MAX);

            for _ in 0..1000000 {
                let a = rng.sample(range);
                let b = rng.sample(range);
                assert_eq!(
                    a.wrapping_add(b),
                    (DepictI32::from(a) + DepictI32::from(b)).into()
                );
            }
        }
    }

    mod sub {
        use crate::i32::DepictI32;

        #[test]
        fn test_subtraction_without_overflow_plus_plus() {
            let a: DepictI32 = 20.into();
            let b: DepictI32 = 10.into();
            let result: DepictI32 = a - b;
            assert_eq!(Into::<i32>::into(result), 10);
        }

        #[test]
        fn test_subtraction_without_overflow_minus_plus() {
            let a: DepictI32 = (-20).into();
            let b: DepictI32 = 10.into();
            let result: DepictI32 = a - b;
            assert_eq!(Into::<i32>::into(result), (-30));
        }

        #[test]
        fn test_subtraction_without_overflow_plus_minus() {
            let a: DepictI32 = 20.into();
            let b: DepictI32 = (-10).into();
            let result: DepictI32 = a - b;
            assert_eq!(Into::<i32>::into(result), 30);
        }

        #[test]
        fn test_subtraction_without_overflow_minus_minus() {
            let a: DepictI32 = (-10).into();
            let b: DepictI32 = (-20).into();
            let result: DepictI32 = a - b;
            assert_eq!(Into::<i32>::into(result), 10);
        }

        #[test]
        fn test_subtraction_with_overflow_plus_minus() {
            for diff_1 in -10..10 {
                for diff_2 in -10..10 {
                    let _a = std::i32::MAX.wrapping_add(diff_1);
                    let _b = std::i32::MIN.wrapping_add(diff_2);
                    let a: DepictI32 = _a.into();
                    let b: DepictI32 = _b.into();
                    let result = a - b;
                    assert_eq!(Into::<i32>::into(result), _a.wrapping_sub(_b),);
                }
            }
        }

        #[test]
        fn test_subtraction_with_underflow_minus_plus() {
            for diff_1 in -10..10 {
                for diff_2 in -10..10 {
                    let _a = std::i32::MIN.wrapping_add(diff_1);
                    let _b = std::i32::MAX.wrapping_add(diff_2);
                    let a: DepictI32 = _a.into();
                    let b: DepictI32 = _b.into();
                    let result = a - b;
                    assert_eq!(Into::<i32>::into(result), _a.wrapping_sub(_b),);
                }
            }
        }

        use rand::distributions::Uniform;
        use rand::Rng;

        #[test]
        fn test_subtraction_consistency_with_i32() {
            let mut rng = rand::thread_rng();
            let range = Uniform::new_inclusive(std::i32::MIN, std::i32::MAX);

            for _ in 0..1000000 {
                let a = rng.sample(range);
                let b = rng.sample(range);
                assert_eq!(
                    a.wrapping_sub(b),
                    (DepictI32::from(a) - DepictI32::from(b)).into()
                );
            }
        }
    }

    mod mul {
        use crate::i32::DepictI32;

        #[test]
        fn test_multiplication_without_overflow_plus_plus() {
            let a: DepictI32 = 10.into();
            let b: DepictI32 = 20.into();
            let result: DepictI32 = a * b;
            assert_eq!(Into::<i32>::into(result), 200);
        }

        #[test]
        fn test_multiplication_without_overflow_minus_plus() {
            let a: DepictI32 = (-10).into();
            let b: DepictI32 = 20.into();
            let result: DepictI32 = a * b;
            assert_eq!(Into::<i32>::into(result), (-200));
        }

        #[test]
        fn test_multiplication_without_overflow_plus_minus() {
            let a: DepictI32 = 10.into();
            let b: DepictI32 = (-20).into();
            let result: DepictI32 = a * b;
            assert_eq!(Into::<i32>::into(result), (-200));
        }

        #[test]
        fn test_multiplication_without_overflow_minus_minus() {
            let a: DepictI32 = (-10).into();
            let b: DepictI32 = (-20).into();
            let result: DepictI32 = a * b;
            assert_eq!(Into::<i32>::into(result), 200);
        }

        #[test]
        fn test_multiplication_with_overflow_plus_plus() {
            for diff_1 in -10..10 {
                for diff_2 in -10..10 {
                    let _a = std::i32::MAX.wrapping_add(diff_1);
                    let _b = std::i32::MAX.wrapping_add(diff_2);
                    println!("a: {}, b: {}", _a, _b);

                    let a: DepictI32 = _a.into();
                    let b: DepictI32 = _b.into();
                    let result = a * b;
                    assert_eq!(Into::<i32>::into(result), _a.wrapping_mul(_b),);
                }
            }
        }
    }
}
