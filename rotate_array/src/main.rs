pub fn max_profit(k: i32, mut prices: Vec<i32>) -> Vec<i32> {
    let n = prices.len();
    let k = (k as usize) % n; 

    prices.reverse(); 
    prices[..k].reverse(); 
    prices[k..].reverse();
    prices
}

fn main() {
    let test = vec![1,2,3,4,5,6,7];
    let result = max_profit(3, test);
    println!("{:?}", result);
}