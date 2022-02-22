use core::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Co2(f32);

impl Add for Co2 {
    type Output = Co2;

    fn add(self, rhs: Self) -> Self::Output {
        Co2(self.0 + rhs.0)
    }
}

impl Co2 {
    pub fn new(n: f32) -> Self {
        Self(n)
    }

    pub fn div(&self, n: f32) -> Self {
        Self::new(self.0 / n)
    }

    pub fn is_valid(&self) -> bool {
        400.0 <= self.0 && self.0 <= 8192.0
    }
}

impl AsRef<f32> for Co2 {
    fn as_ref(&self) -> &f32 {
        &self.0
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for Co2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
