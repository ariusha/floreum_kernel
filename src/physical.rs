use crate::language::this::LIMINE_MEMORY_HHDM_RESPONSE_NONE;
use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use limine::request::HhdmRequest;
#[used]
#[unsafe(link_section = ".requests")]
static HHDM: HhdmRequest = HhdmRequest::new();
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Physical(u64);
impl Physical {
    pub unsafe fn from_raw(raw: u64) -> Self {
        Physical(raw)
    }
    pub fn into_raw(&self) -> u64 {
        self.0.clone()
    }
    pub fn to_virtual(&self) -> *const () {
        (HHDM
            .response()
            .expect(LIMINE_MEMORY_HHDM_RESPONSE_NONE)
            .offset
            + self.0) as *const ()
    }
}
impl From<u64> for Physical {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<Physical> for u64 {
    fn from(value: Physical) -> Self {
        value.0
    }
}
impl Add for Physical {
    type Output = Physical;
    fn add(self, rhs: Self) -> Self::Output {
        Physical(self.0 + rhs.0)
    }
}
impl Sub for Physical {
    type Output = Physical;
    fn sub(self, rhs: Self) -> Self::Output {
        Physical(self.0 - rhs.0)
    }
}
impl Mul for Physical {
    type Output = Physical;
    fn mul(self, rhs: Self) -> Self::Output {
        Physical(self.0 * rhs.0)
    }
}
impl AddAssign for Physical {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}
impl SubAssign for Physical {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}
impl MulAssign for Physical {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}
