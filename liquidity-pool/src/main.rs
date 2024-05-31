mod decimal;
use decimal::{Decimal, DecimalInit};

#[derive(Debug)]
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
        if price<0.0 || min_fee <0.0 || max_fee <0.0 || liquidity_target < 0.0 {return Err("Invalid Argument".to_string())}
        if max_fee < min_fee { 
            return Err("minimal fee must be lower than maximal..".to_string())
        }
        if max_fee > 100.0 {
            return Err("great deal!".to_string())
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
        if token_amount < 0.0 {return Err("Invalid Argument".to_string())}
        let token_amount = Decimal::init(token_amount);
        let potential_denominator  = self.token_amount + (self.price * self.staked_token_amount);
        if potential_denominator == (Decimal::init(0)) { //only occurs when pool is empty
            self.token_amount += token_amount;
            self.lp_token_amount = token_amount;
            return Ok(token_amount)
        }
        let lp_new_tokens =  token_amount * self.lp_token_amount / potential_denominator ;
        self.lp_token_amount += lp_new_tokens;
        self.token_amount += token_amount;
        Ok(lp_new_tokens)
    }


    pub fn remove_liquidity(&mut self, lp_token_amount : f64) -> Result<(Decimal, Decimal), String>{
        if lp_token_amount < 0.0 {return Err("Invalid Argument".to_string())}
        let lp_token_amount = Decimal::init(lp_token_amount);
        if lp_token_amount > self.lp_token_amount{
            return Err("It is not possible to remove more than everything..".to_string())
        }
        let percentage_amount = lp_token_amount/self.lp_token_amount;
        let token_amount = percentage_amount * self.token_amount;
        let staked_token_amount = percentage_amount * self.staked_token_amount;
        self.lp_token_amount -= lp_token_amount;
        Ok((token_amount,staked_token_amount))
    }

    pub fn swap(&mut self, staked_token_amount: f64) -> Result<Decimal,String>{
        if staked_token_amount < 0.0 {return Err("Invalid Argument".to_string())}
        let staked_token_amount = Decimal::init(staked_token_amount);
        let token_amount = staked_token_amount * self.price;
        if token_amount  > self.token_amount {
            return Err("Transaction not possible! Not enough liquidity provided.".to_string())
        }
        self.staked_token_amount += staked_token_amount;
        self.token_amount -= token_amount;
        let fee = if self.token_amount  < self.liquidity_target 
                {self.max_fee - (self.max_fee - self.min_fee)*(self.token_amount/self.liquidity_target)} 
            else
                {self.min_fee};
        let res = token_amount  - (fee*token_amount);
        self.token_amount += fee*token_amount;        
        Ok(res)
    }
}
fn main() {
    let mut lp_pool = LpPool::init(1.5, 0.1, 9.0, 90.0, 7).unwrap();
    println!("Created Liqudity Pool!");
    let gained_lp_tokens = lp_pool.add_liquidity(100.0).unwrap();
    println!("Added Liqudity to the pool, gained {} lp tokens.",gained_lp_tokens );
    let exchanged_tokens = lp_pool.swap(6.0).unwrap();
    println!("Exchange staked tokens. Gained  {} tokens.", exchanged_tokens);
    let gained_lp_tokens = lp_pool.add_liquidity(10.0).unwrap();
    println!("Added Liqudity to the pool, gained {} lp tokens.",gained_lp_tokens );
    let exchanged_tokens = lp_pool.swap(30.0).unwrap();
    println!("Exchange staked tokens. Gained  {} tokens.", exchanged_tokens);
    let removed_tokens = lp_pool.remove_liquidity(100.0).unwrap();
    println!("Removed Liqudity. Gained {} tokens and {} staked tokens", removed_tokens.0, removed_tokens.1);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_initialization(){
        let lp_pool = LpPool::init(1.5, 0.1, 9.0, 90.0, 7).unwrap();
        assert_eq!(lp_pool.price,Decimal::init(1.5));
        assert_eq!(lp_pool.min_fee,Decimal::init(0.001));
        assert_eq!(lp_pool.max_fee,Decimal::init(0.09));
        assert_eq!(lp_pool.liquidity_target,Decimal::init(90));
        assert_eq!(lp_pool.staked_token_amount,Decimal::init(0));
        assert_eq!(lp_pool.token_amount,Decimal::init(0));
        assert_eq!(lp_pool.lp_token_amount,Decimal::init(0));
    }

    #[test]
    #[should_panic]
    fn pool_init_error(){
        LpPool::init(2.0, 8.0, 3.0, 100.0, 5).unwrap();
    }
    #[test]
    fn adding_liquidity(){
        let mut lp_pool = LpPool::init(1.0, 0.0, 4.0, 14.0, 3).unwrap();
        assert_eq!(lp_pool.add_liquidity(20.0).unwrap(), Decimal::init(20));
        assert_eq!(lp_pool.add_liquidity(10.0).unwrap(), Decimal::init(10));
        lp_pool.swap(15.0).unwrap();
        assert_eq!(lp_pool.add_liquidity(10.0).unwrap(), Decimal::init(10));
        lp_pool.swap(20.0).unwrap();
        assert_eq!(lp_pool.add_liquidity(10.0).unwrap(), Decimal::init(9.871));
    }

    #[test]
    fn swapping(){
        let mut lp_pool = LpPool::init(2.0, 10.0, 50.0, 100.0, 2).unwrap();
        lp_pool.add_liquidity(120.0).unwrap();
        assert_eq!(lp_pool.swap(5.0).unwrap(),Decimal::init(9));
        assert_eq!(lp_pool.swap(30.0).unwrap(),Decimal::init(42));
    }

    #[should_panic]
    #[test]
    fn swapping_error(){
        let mut lp_pool = LpPool::init(2.0, 10.0, 50.0, 100.0, 2).unwrap();
        lp_pool.add_liquidity(10.0).unwrap();
        lp_pool.swap(7.0).unwrap();
    }

    #[test]
    fn removing_liquidity(){
        let mut lp_pool = LpPool::init(2.0, 50.0, 70.0, 10.0, 2).unwrap();
        lp_pool.add_liquidity(200.0).unwrap();
        lp_pool.swap(50.0).unwrap();
        assert_eq!(lp_pool.remove_liquidity(100.0).unwrap(),(Decimal::init(75),Decimal::init(25)))
    }
}

