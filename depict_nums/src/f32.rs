use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DepictF32(f64);

impl From<f32> for DepictF32 {
    fn from(f: f32) -> Self {
        DepictF32(f as f64)
    }
}

impl From<DepictF32> for f32 {
    fn from(f: DepictF32) -> Self {
        f.0 as f32
    }
}

impl std::ops::Add for DepictF32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        println!("calc_e: {:?}", self.calc_e());

        // DepictF32(self.0 + rhs.0)
        DepictF32(0f64)
    }
}

impl DepictF32 {
    // 符号部の計算
    pub fn calc_sign(&self) -> bool {
        if self.0 == 0.0 {
            if self.0.to_string().chars().nth(0).unwrap() == '-' {
                return true;
            } else {
                return false;
            }
        }
        self.0 < 0.0
    }

    // 指数部の計算
    pub fn calc_e(&self) -> (Vec<bool>, f64) {
        // 2で割っていく
        let mut f = self.0;
        if f == std::f64::INFINITY {
            return (vec![true; 11], 1024f64);
        }
        if f == std::f64::NEG_INFINITY {
            return (vec![true; 11], 1024f64);
        }
        if f.is_nan() {
            return (vec![true; 11], 1024f64);
        }
        if f == 0.0 {
            return (vec![false; 11], -1023f64);
        }
        let mut e: f64 = 0.0;
        let e = if f < 0.0 {
            f = -f;
            if f >= 2.0 {
                while f >= 2.0 {
                    f /= 2.0;
                    e += 1.0;
                }
                e
            } else {
                while f < 1.0 {
                    f *= 2.0;
                    e -= 1.0;
                }
                e
            }
        } else {
            if f >= 2.0 {
                while f >= 2.0 {
                    f /= 2.0;
                    e += 1.0;
                }
                e
            } else {
                while f < 1.0 {
                    f *= 2.0;
                    e -= 1.0;
                }
                e
            }
        };
        let e = e + 1023.0;
        // println!("e: {:?}", e);
        let mut e_bits = vec![];
        // bit演算は使わない
        let mut b = e;
        for _ in 0..11 {
            e_bits.push(b % 2.0 == 1.0);
            b = (b / 2.0).floor();
        }
        e_bits.reverse();
        (e_bits, e)
    }

    pub fn calc_f(&self) -> Vec<bool> {
        let mut f = self.0;
        if f.is_nan() {
            return vec![false; 52];
        }
        if f == std::f64::INFINITY {
            return vec![false; 52];
        }
        if f == std::f64::NEG_INFINITY {
            return vec![false; 52];
        }
        if f == 0.0 || f == -0.0 {
            return vec![false; 52];
        }
        let (_, e) = self.calc_e();
        let e = e - 1023.0;

        // e(指数)を用いて正規化する
        // println!("{:?}", e);
        // println!("{:?}", f);
        if f < 0.0 {
            f = -f;
        }
        f = f / 2.0f64.powi(e as i32);
        f -= 1.0;

        let mut f_bits = Vec::new();
        // let mut count = 0;

        // 2進数にする
        while f != 0.0 {
            f *= 2.0;
            if f >= 1.0 {
                f_bits.push(true);
                f -= 1.0;
            } else {
                f_bits.push(false);
            }

            // count += 1;
            // if count > 52 {
            //     break;
            // }
        }

        for _ in f_bits.len()..52 {
            f_bits.push(false);
        }

        f_bits
    }

    // bitの計算
    pub fn get_bits_by_shift(&self) -> Vec<bool> {
        // println!("sl: {:?}", self.0);
        let f = self.0;
        let f_bits = f.to_bits();
        let mut bits = vec![];
        for i in 0..64 {
            bits.push(f_bits & (1 << i) != 0);
        }
        bits.reverse();
        // println!("{:b}", f_bits);
        // for i in 0..64 {
        //     print!("{:?}", if bits[i] { 1 } else { 0 });
        // }
        // println!();
        bits
    }

    // 符号部の計算
    pub fn get_sign_by_shift(&self) -> bool {
        let bits = self.get_bits_by_shift();
        bits[0]
    }

    // 指数部の計算
    pub fn get_e_by_shift(&self) -> Vec<bool> {
        let bits = self.get_bits_by_shift();
        bits[1..12].to_vec()
    }

    // 仮数部の計算
    pub fn get_f_by_shift(&self) -> Vec<bool> {
        let bits = self.get_bits_by_shift();
        if self.0.is_nan() {
            return vec![false; 52];
        }
        return bits[12..64].to_vec();
    }
}

#[cfg(test)]
mod test {
    mod get_sign_by_shift {
        use super::super::DepictF32;

        #[test]
        fn test_get_sign() {
            for i in 0..32 {
                let a = DepictF32::from(2.0f32.powi(i));
                assert_eq!(a.get_sign_by_shift(), false);
            }
        }

        #[test]
        fn test_get_sign_2() {
            for i in 0..32 {
                let a = DepictF32::from(-2.0f32.powi(i));
                println!("{:?}", a);
                assert_eq!(a.get_sign_by_shift(), true);
            }
        }

        #[test]
        fn test_get_sign_3() {
            let a = DepictF32::from(0.0);
            assert_eq!(a.get_sign_by_shift(), false);
        }

        #[test]
        fn test_get_sign_4() {
            let a = DepictF32::from(-0.0);
            assert_eq!(a.get_sign_by_shift(), true);
        }

        #[test]
        fn test_get_sign_5() {
            let a = DepictF32::from(f32::INFINITY);
            assert_eq!(a.get_sign_by_shift(), false);
        }

        #[test]
        fn test_get_sign_6() {
            let a = DepictF32::from(f32::NEG_INFINITY);
            assert_eq!(a.get_sign_by_shift(), true);
        }

        #[test]
        fn test_get_sign_7() {
            let a = DepictF32::from(f32::NAN);
            assert_eq!(a.get_sign_by_shift(), false);
        }

        // これは保証されていないため機能しない
        // #[test]
        // fn test_get_sign_8() {
        //     let a = DepictF32::from(-f32::NAN);
        //     assert_eq!(a.get_sign_by_shift(), false);
        // }
    }

    mod get_e_by_shift {
        use super::super::DepictF32;

        #[test]
        fn test_get_e() {
            for i in 0..32 {
                let a = DepictF32::from(2.0f32.powi(i));
                let e = a.get_e_by_shift();
                let mut answer = vec![false; 11];
                let i = i + 1023;
                for j in 0..11 {
                    answer[j] = i & (1 << (10 - j)) != 0;
                }
                assert_eq!(e, answer);
            }
        }

        #[test]
        fn test_get_e_2() {
            for i in 0..32 {
                let a = DepictF32::from(-2.0f32.powi(i));
                let e = a.get_e_by_shift();
                let mut answer = vec![false; 11];
                let i = i + 1023;
                for j in 0..11 {
                    answer[j] = i & (1 << (10 - j)) != 0;
                }
                assert_eq!(e, answer);
            }
        }
    }

    mod get_f_by_shift {
        use super::super::DepictF32;

        #[test]
        fn test_get_f() {
            for i in 0..32 {
                let a = DepictF32::from(2.0f32.powi(i));
                assert_eq!(a.get_f_by_shift(), vec![false; 52]);
            }
        }

        #[test]
        fn test_get_f_2() {
            for i in 0..32 {
                let a = DepictF32::from(-2.0f32.powi(i));
                assert_eq!(a.get_f_by_shift(), vec![false; 52]);
            }
        }

        #[test]
        fn test_get_f_3() {
            let a = DepictF32::from(0.0);
            assert_eq!(a.get_f_by_shift(), vec![false; 52]);
        }

        #[test]
        fn test_get_f_4() {
            let a = DepictF32::from(-0.0);
            assert_eq!(a.get_f_by_shift(), vec![false; 52]);
        }

        #[test]
        fn test_get_f_5() {
            let a = DepictF32::from(f32::INFINITY);
            assert_eq!(a.get_f_by_shift(), vec![false; 52]);
        }

        #[test]
        fn test_get_f_6() {
            let a = DepictF32::from(f32::NEG_INFINITY);
            assert_eq!(a.get_f_by_shift(), vec![false; 52]);
        }

        #[test]
        fn test_get_f_7() {
            let a = DepictF32::from(f32::NAN);
            assert_eq!(a.get_f_by_shift(), vec![false; 52]);
        }

        #[test]
        fn test_get_f_8() {
            let a = DepictF32::from(-f32::NAN);
            assert_eq!(a.get_f_by_shift(), vec![false; 52]);
        }

        use rand::distributions::Uniform;
        use rand::Rng;

        #[test]
        fn test_get_f_9() {
            let mut rng = rand::thread_rng();
            let range: Uniform<u32> = Uniform::new(std::u32::MIN, std::u32::MAX);
            for _ in 0..100000 {
                let low = rng.sample(range);
                let a = DepictF32::from(f32::from_bits(low));
                if a.0.abs() < i32::MAX as f64 {
                    let t = a.get_f_by_shift();
                    let b = f32::from_bits(low) * 2.0;
                    let c = DepictF32::from(b);
                    let u = c.get_f_by_shift();
                    assert_eq!(t, u);
                }
            }
        }
    }

    mod calc_sign {
        #[test]
        fn test_calc_sign() {
            for i in 0..32 {
                let a = super::super::DepictF32::from(2.0f32.powi(i));
                assert_eq!(a.calc_sign(), false);
            }
        }

        #[test]
        fn test_calc_sign_2() {
            for i in 0..32 {
                let a = super::super::DepictF32::from(-2.0f32.powi(i));
                assert_eq!(a.calc_sign(), true);
            }
        }

        #[test]
        fn test_calc_sign_3() {
            let a = super::super::DepictF32::from(0.0);
            assert_eq!(a.calc_sign(), false);
        }

        #[test]
        fn test_calc_sign_4() {
            let a = super::super::DepictF32::from(-0.0);
            println!("## {:?}", -0.0);
            assert_eq!(a.calc_sign(), true);
        }

        #[test]
        fn test_calc_sign_5() {
            let a = super::super::DepictF32::from(f32::INFINITY);
            assert_eq!(a.calc_sign(), false);
        }

        #[test]
        fn test_calc_sign_6() {
            let a = super::super::DepictF32::from(f32::NEG_INFINITY);
            assert_eq!(a.calc_sign(), true);
        }

        #[test]
        fn test_calc_sign_7() {
            let a = super::super::DepictF32::from(f32::NAN);
            assert_eq!(a.calc_sign(), false);
        }
    }

    mod calc_e {
        use super::super::DepictF32;

        #[test]
        fn test_calc_e() {
            for i in 0..32 {
                let a = DepictF32::from(2.0f32.powi(i));
                let (e_calc, _) = a.calc_e();
                let e_shift = a.get_e_by_shift();
                assert_eq!(e_calc, e_shift);
            }
        }

        #[test]
        fn test_calc_e_2() {
            for i in 0..32 {
                let a = DepictF32::from(-2.0f32.powi(i));
                let (e_calc, _) = a.calc_e();
                let e_shift = a.get_e_by_shift();
                assert_eq!(e_calc, e_shift);
            }
        }

        #[test]
        fn test_calc_e_3() {
            let a = DepictF32::from(0.0);
            let (e_calc, _) = a.calc_e();
            let e_shift = a.get_e_by_shift();
            assert_eq!(e_calc, e_shift);
        }

        #[test]
        fn test_calc_e_4() {
            let a = DepictF32::from(-0.0);
            let (e_calc, _) = a.calc_e();
            let e_shift = a.get_e_by_shift();
            assert_eq!(e_calc, e_shift);
        }

        #[test]
        fn test_calc_e_5() {
            let a = DepictF32::from(f32::INFINITY);
            let (e_calc, _) = a.calc_e();
            let e_shift = a.get_e_by_shift();
            assert_eq!(e_calc, e_shift);
        }

        #[test]
        fn test_calc_e_6() {
            let a = DepictF32::from(f32::NEG_INFINITY);
            let (e_calc, _) = a.calc_e();
            let e_shift = a.get_e_by_shift();
            assert_eq!(e_calc, e_shift);
        }

        use rand::distributions::Uniform;
        use rand::Rng;

        #[test]
        fn test_calc_e_7() {
            let mut rng = rand::thread_rng();
            let range: Uniform<u32> = Uniform::new(std::u32::MIN, std::u32::MAX);
            for _ in 0..100000 {
                let low = rng.sample(range);
                let a = DepictF32::from(f32::from_bits(low));
                let (e_calc, _) = a.calc_e();
                let e_shift = a.get_e_by_shift();
                assert_eq!(e_calc, e_shift);
            }
        }
    }

    mod calc_f {
        #[test]
        fn test_calc_f() {
            for i in 0..32 {
                let a = super::super::DepictF32::from(2.0f32.powi(i));
                assert_eq!(a.calc_f(), vec![false; 52]);
            }
        }

        #[test]
        fn test_calc_f_2() {
            for i in 0..32 {
                let a = super::super::DepictF32::from(-2.0f32.powi(i));
                assert_eq!(a.calc_f(), vec![false; 52]);
            }
        }

        #[test]
        fn test_calc_f_3() {
            let a = super::super::DepictF32::from(0.0);
            assert_eq!(a.calc_f(), vec![false; 52]);
        }

        #[test]
        fn test_calc_f_4() {
            let a = super::super::DepictF32::from(-0.0);
            assert_eq!(a.calc_f(), vec![false; 52]);
        }

        #[test]
        fn test_calc_f_5() {
            let a = super::super::DepictF32::from(f32::INFINITY);
            assert_eq!(a.calc_f(), vec![false; 52]);
        }

        #[test]
        fn test_calc_f_6() {
            let a = super::super::DepictF32::from(f32::NEG_INFINITY);
            assert_eq!(a.calc_f(), vec![false; 52]);
        }

        #[test]
        fn test_calc_f_7() {
            let a = super::super::DepictF32::from(f32::NAN);
            assert_eq!(a.calc_f(), vec![false; 52]);
        }

        use rand::distributions::Uniform;
        use rand::Rng;

        #[test]
        fn test_calc_f_8() {
            let mut rng = rand::thread_rng();
            let range: Uniform<u32> = Uniform::new(std::u32::MIN, std::u32::MAX);
            for _ in 0..100000 {
                let low = rng.sample(range);
                let a = super::super::DepictF32::from(f32::from_bits(low));
                if a.0.abs() < i32::MAX as f64 {
                    let t = a.calc_f();
                    let b = f32::from_bits(low) * 2.0;
                    let c = super::super::DepictF32::from(b);
                    let u = c.calc_f();
                    assert_eq!(t, u);
                }
            }
        }

        #[test]
        fn test_calc_f_9() {
            let mut rng = rand::thread_rng();
            let range: Uniform<u32> = Uniform::new(std::u32::MIN, std::u32::MAX);
            for _ in 0..100000 {
                let low = rng.sample(range);
                let a = super::super::DepictF32::from(f32::from_bits(low));
                let t = a.calc_f();
                let b = super::super::DepictF32::from(f32::from_bits(low));
                let u = b.get_f_by_shift();
                assert_eq!(t, u);
            }
        }
    }

    // mod add {
    //     use super::super::DepictF32;

    //     #[test]
    //     fn test_add() {
    //         let a = DepictF32::from(1.0);
    //         let b = DepictF32::from(2.0);
    //         let c = a + b;
    //         assert_eq!(c, DepictF32::from(3.0));
    //     }
    // }
}
