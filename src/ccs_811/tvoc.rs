use core::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Tvoc(pub f32);

impl Add for Tvoc {
    type Output = Tvoc;

    fn add(self, rhs: Self) -> Self::Output {
        Tvoc(self.0 + rhs.0)
    }
}

impl Tvoc {
    pub fn div(&self, n: f32) -> Self {
        Tvoc(self.0 / n)
    }
}

impl ToString for Tvoc {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
