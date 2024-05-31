use once_cell::sync::OnceCell;
use std::{fmt::{Display, Formatter}, ops::{AddAssign, SubAssign, Add, Sub, Mul, Div}};


pub trait DecimalInit<T> {
    fn init(number: T) -> Self;
}


#[derive(Copy, Clone, Eq, PartialOrd, Ord)]
pub struct Decimal {
    pub number: u64,
}
static SCALE: OnceCell<u64> = OnceCell::new();

impl DecimalInit<u64> for Decimal{
    fn init(number : u64) -> Self{
        let scale : u64 = Decimal::get_scale();
        Decimal{number : number * scale}
    }
}

impl DecimalInit<f64> for Decimal{
    fn init(number : f64) -> Self{
        let scale : u64 = Decimal::get_scale();
        Decimal{number : (number * scale as f64 ) as u64}
    }
}

impl Decimal {

    pub fn initialize_scale(scale: u64) {
        SCALE.set(10u64.pow(scale as u32)).expect("SCALE can only be set once");
    }

    pub fn get_scale() -> u64 {
        *SCALE.get().expect("SCALE must be initialized before use")
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { number: self.number + other.number }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { number: self.number - other.number }
    }
}

impl Mul<Decimal> for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let scale = Decimal::get_scale();
        Self { number: self.number * other.number / scale }
    }
}

impl Mul<u64> for Decimal {
    type Output = Self;

    fn mul(self, other: u64) -> Self {
        Self { number: self.number * other }
    }
}

impl Div for Decimal {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let scale = Decimal::get_scale();
        Self { number: (self.number * scale) / other.number }
    }
}

impl AddAssign for Decimal {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Decimal {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let scale = Decimal::get_scale();
        let res = self.number as f64 / scale as f64;
        write!(f, "{}", res)
    }
}