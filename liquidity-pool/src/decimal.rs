use std::{fmt::{Display, Formatter}, ops::{AddAssign, SubAssign}};

#[derive(Copy, Clone, Eq, PartialOrd, Ord)]
pub struct Decimal{
    pub number : u64,
    pub scale : u64,
}

impl PartialEq for Decimal{
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.scale == other.scale
    }
}
impl std::ops::Add for Decimal{
    type Output = Self;

    fn add(self, other : Self) -> Self{
        Self{number: self.number + other.number, scale: self.scale}
    }
}

impl std::ops::Sub for Decimal{
    type Output =  Self;

    fn sub(self, other: Self) -> Self {
        Self { number: self.number - other.number, scale: self.scale }
    }
}
impl std::ops::Mul<Decimal> for Decimal{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self{number: self.number * other.number / self.scale, scale: self.scale}
    }
}

impl std::ops::Mul<u64> for Decimal{
    type Output = Self;

    fn mul(self, other: u64) -> Self {
        Self{number: self.number * other, scale: self.scale}
    }
}
impl std::ops::Div for Decimal{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self{number: (self.number * self.scale)/other.number, scale: self.scale}
    }
}

impl AddAssign for Decimal{

    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl SubAssign for Decimal{

    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
 
    }
}

impl Display for Decimal{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = self.number as f64 / self.scale as f64 ;
        write!(f, "{}", res)?;
        Ok(())  
    }
}