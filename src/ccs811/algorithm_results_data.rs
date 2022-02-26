use crate::*;

/// CSS811 による測定値演算結果
pub struct AlgorithmResultsData([u8; 6]);

impl AlgorithmResultsData {
    pub fn new(data: [u8; 6]) -> Self {
        Self(data)
    }

    pub fn status(&self) -> Ccs811Result<Status> {
        Status::try_new(self.0[4])
    }

    pub fn co2(&self) -> Co2 {
        Co2::new(((self.0[0] as u16) << 8 | self.0[1] as u16).into())
    }

    pub fn tvoc(&self) -> Tvoc {
        Tvoc::new(((self.0[2] as u16) << 8 | self.0[3] as u16).into())
    }
}
