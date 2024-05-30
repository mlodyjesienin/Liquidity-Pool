use std::{fmt::{Display, Formatter}, ops::{AddAssign, SubAssign}};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Decimal{
    number : u64,
    scale : u64,
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
        Self { number: self.number + other.number, scale: self.scale }
    }
}
impl std::ops::Mul for Decimal{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self{number: self.number * other.number / self.scale, scale: self.scale}
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
        let res = self.number / self.scale;
        write!(f, "{}", res)?;
        Ok(())
    }
}
struct LpPool {
    price : Decimal,
    token_amount : Decimal,
    staked_token_amount : Decimal,
    lp_token_amount : Decimal,
    liquidity_target : Decimal,
    min_fee : Decimal,
    max_fee : Decimal,
    scale: u64,
}

impl LpPool {
    pub fn init (price: f64, min_fee : f64, max_fee : f64, liquidity_target : f64, scale: u64) -> Result<Self,String>{
        if max_fee < min_fee { 
            return Err("Error".to_string())
        }
        if max_fee > 100.0 {
            return Err("ERROR".to_string())
        }
        
        let price = Decimal{number: (price*scale as f64) as u64, scale: scale};
        let min_fee = Decimal{number: (min_fee*scale as f64) as u64, scale: scale};
        let max_fee = Decimal{number: (max_fee*scale as f64) as u64, scale: scale};
        let liquidity_target = Decimal{number: (liquidity_target * scale as f64) as u64, scale: scale};

        Ok(LpPool{price : price, min_fee : min_fee, token_amount : Decimal{number: 0, scale: scale}, staked_token_amount : Decimal{number: 0, scale: scale} , 
            lp_token_amount : Decimal{number: 0, scale: scale}, liquidity_target : liquidity_target, max_fee : max_fee, scale: 10u64.pow(scale as u32)})
    }

    pub fn add_liquidity(&mut self, token_amount: f64) -> Result<Decimal, String>{
        let token_amount = Decimal{number: (token_amount * self.scale as f64) as u64, scale: self.scale};
        let potential_denominator  = self.token_amount + (self.price * self.lp_token_amount);
        if potential_denominator == (Decimal{number: 0, scale: self.scale}) { //only occurs when pool is empty
            self.token_amount += token_amount;
            self.lp_token_amount = token_amount;
            println!("gained {} lp tokens", self.lp_token_amount.number);
            return Ok(Decimal{number: token_amount.number,scale: self.scale})
        }
        let lp_new_tokens =  token_amount / potential_denominator ;
        self.lp_token_amount += lp_new_tokens;
        self.token_amount += token_amount;
        println!("gained {} lp tokens", lp_new_tokens);
        Ok(lp_new_tokens)
    }


    pub fn remove_liquidity(&mut self, lp_token_amount : f64) -> Result<(Decimal, Decimal), String>{
        let lp_token_amount = Decimal{number: (lp_token_amount* self.scale as f64) as u64, scale: self.scale};
        if lp_token_amount > self.lp_token_amount{
            return Err("What is Happening?!".to_string())
        }
        let percentage_amount = lp_token_amount/self.lp_token_amount;
        let token_amount = percentage_amount * self.token_amount;
        let staked_token_amount = percentage_amount * self.staked_token_amount;
        self.lp_token_amount -= lp_token_amount;
        println!("removed {} tokens and {} staked tokens ", token_amount, staked_token_amount);
        Ok((token_amount,staked_token_amount))
    }

    pub fn swap(&mut self, staked_token_amount: f64) -> Result<Decimal,String>{
        let staked_token_amount = Decimal{number: (staked_token_amount* self.scale as f64) as u64, scale: self.scale};
        let token_amount = staked_token_amount * self.price;
        if token_amount  > self.token_amount {
            return Err("Transaction not possible!".to_string())
        }
        self.token_amount -= token_amount;
        self.staked_token_amount += staked_token_amount;
        let fee = if self.token_amount < self.liquidity_target {self.max_fee - (self.max_fee - self.min_fee)*(self.token_amount/self.liquidity_target)} 
                else{self.min_fee};
        let res = token_amount - fee;        
        println!("swapped for {} token amount", res);
        Ok(res)
    }
}
fn main() {
    println!("Hello, world!");
    let mut lp_pool = LpPool::init(1.5, 0.1, 9.0, 90.0, 5).unwrap();
    lp_pool.add_liquidity(100.0).unwrap();
    println!("tokens: {}",lp_pool.token_amount);
    println!("lp tokens: {}",lp_pool.lp_token_amount);
    lp_pool.swap(6.0).unwrap();


}
