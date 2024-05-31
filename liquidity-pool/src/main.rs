mod decimal;
use decimal::{Decimal, DecimalInit};

struct LpPool {
    price : Decimal,
    token_amount : Decimal,
    staked_token_amount : Decimal,
    lp_token_amount : Decimal,
    liquidity_target : Decimal,
    min_fee : Decimal,
    max_fee : Decimal,
}

impl LpPool {
    pub fn init (price: f64, min_fee : f64, max_fee : f64, liquidity_target : f64, scale: u64) -> Result<Self,String>{
        if max_fee < min_fee { 
            return Err("Error".to_string())
        }
        if max_fee > 100.0 {
            return Err("ERROR".to_string())
        }
        Decimal::initialize_scale(scale);

        let price = Decimal::init(price);
        let min_fee = Decimal::init(0.01 * min_fee);
        let max_fee = Decimal::init(0.01 * max_fee);
        let liquidity_target = Decimal::init(liquidity_target);
        Ok(LpPool{price : price, min_fee : min_fee, token_amount : Decimal::init(0), staked_token_amount : Decimal::init(0), 
            lp_token_amount : Decimal::init(0), liquidity_target : liquidity_target, max_fee : max_fee})
    }

    pub fn add_liquidity(&mut self, token_amount: f64) -> Result<Decimal, String>{
        let token_amount = Decimal::init(token_amount);
        let potential_denominator  = self.token_amount + (self.price * self.staked_token_amount);
        println!("token: {}, lp.token {}",self.token_amount, self.staked_token_amount);
        if potential_denominator == (Decimal::init(0)) { //only occurs when pool is empty
            self.token_amount += token_amount;
            self.lp_token_amount = token_amount;
            println!("gained {} lp tokens", self.lp_token_amount);
            return Ok(Decimal::init(token_amount.number))
        }
        let lp_new_tokens =  token_amount* 100 as u64 / potential_denominator ;
        self.lp_token_amount += lp_new_tokens;
        self.token_amount += token_amount;
        println!("gained {} lp tokens", lp_new_tokens);
        Ok(lp_new_tokens)
    }


    pub fn remove_liquidity(&mut self, lp_token_amount : f64) -> Result<(Decimal, Decimal), String>{
        let lp_token_amount = Decimal::init(lp_token_amount);
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
        let staked_token_amount = Decimal::init(staked_token_amount);
        let token_amount = staked_token_amount * self.price;
        if token_amount  > self.token_amount {
            return Err("Transaction not possible!".to_string())
        }
        self.staked_token_amount += staked_token_amount;
        self.token_amount -= token_amount;
        println!("maxfee {}, min fee {}, token am {}, target {}", self.max_fee,self.min_fee, self.token_amount, self.liquidity_target);
        let fee = if self.token_amount  < self.liquidity_target 
                {self.max_fee - (self.max_fee - self.min_fee)*(self.token_amount/self.liquidity_target)} 
            else
                {self.min_fee};
        println!("fee: {} ",fee*token_amount);
        let res = token_amount  - (fee*token_amount);
        self.token_amount += fee*token_amount;        
        println!("swapped for {} token amount", res);
        Ok(res)
    }
}
fn main() {
    println!("Hello, world!");
    let mut lp_pool = LpPool::init(1.5, 0.1, 9.0, 90.0, 7).unwrap();
    lp_pool.add_liquidity(100.0).unwrap();
    lp_pool.swap(6.0).unwrap();
    lp_pool.add_liquidity(10.0);
    lp_pool.swap(30.0);
    lp_pool.remove_liquidity(10.0);
}
