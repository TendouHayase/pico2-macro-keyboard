use core::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration(pub u64);

impl Duration {
    pub fn checked_add(self, rhs: Duration) -> Option<Duration> {
        self.0.checked_add(rhs.0).map(Duration)
    }

    pub fn checked_sub(self, rhs: Duration) -> Option<Duration> {
        self.0.checked_sub(rhs.0).map(Duration)
    }

    pub fn checked_mul(self, rhs: u32) -> Option<Duration> {
        self.0.checked_mul(rhs as _).map(Duration)
    }

    pub fn checked_div(self, rhs: u32) -> Option<Duration> {
        self.0.checked_div(rhs as _).map(Duration)
    }
}

impl Add for Duration {
    type Output = Duration;
    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
            .expect("overflow when adding durations")
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Duration {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
            .expect("overflow when subtracting durations")
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for Duration {
    type Output = Duration;
    fn mul(self, rhs: u32) -> Self::Output {
        self.checked_mul(rhs)
            .expect("overflow when multiplying duration by scalar")
    }
}

impl Mul<Duration> for u32 {
    type Output = Duration;
    fn mul(self, rhs: Duration) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<u32> for Duration {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for Duration {
    type Output = Duration;
    fn div(self, rhs: u32) -> Self::Output {
        self.checked_div(rhs)
            .expect("divide by zero error when dividing duration by scalar")
    }
}

impl Div<Duration> for u32 {
    type Output = Duration;
    fn div(self, rhs: Duration) -> Self::Output {
        rhs / self
    }
}

impl DivAssign<u32> for Duration {
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ticks", self.0)
    }
}
