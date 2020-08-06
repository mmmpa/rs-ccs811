use core::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Tvoc(f32);

impl Add for Tvoc {
    type Output = Tvoc;

    fn add(self, rhs: Self) -> Self::Output {
        Tvoc(self.0 + rhs.0)
    }
}

impl Tvoc {
    pub fn new(n: f32) -> Self {
        Self(n)
    }

    pub fn div(&self, n: f32) -> Self {
        Tvoc(self.0 / n)
    }

    pub fn is_valid(&self) -> bool {
        0.0 <= self.0 && self.0 <= 1187.0
    }
}

impl AsRef<f32> for Tvoc {
    fn as_ref(&self) -> &f32 {
        &self.0
    }
}
