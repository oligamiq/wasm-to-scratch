#[derive(Debug, PartialEq, Clone, Copy)]
struct DepictF64(f64);

impl From<f64> for DepictF64 {
    fn from(f: f64) -> Self {
        DepictF64(f)
    }
}

impl From<DepictF64> for f64 {
    fn from(f: DepictF64) -> Self {
        f.0
    }
}

impl std::ops::Add for DepictF64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        DepictF64(self.0 + rhs.0)
    }
}

impl std::ops::Sub for DepictF64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        DepictF64(self.0 - rhs.0)
    }
}

impl std::ops::Mul for DepictF64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        DepictF64(self.0 * rhs.0)
    }
}

impl std::ops::Div for DepictF64 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        DepictF64(self.0 / rhs.0)
    }
}
