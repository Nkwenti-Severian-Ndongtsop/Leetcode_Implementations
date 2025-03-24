
fn highest_profit(prices: Vec<i32>) -> i32 {

    //  5               // 0 
    // [7,1,5,3,6,4]   [7,6,4,3,1]
    let mut max_profit = vec![0];
    let mut actual_length = max_profit.len();
    if prices.len() == 2 && prices[0] < prices[1] {
        return prices[0]
    }
    let mut count = 0;
    for i in 0..prices.len() - 1 {
        for j in i+1..prices.len() - 1 {
            let temp_profit = prices[j] - prices[i]; 
            if temp_profit > max_profit[count] {
                max_profit.push(temp_profit);
                actual_length = max_profit.len();
                count += 1;
            }
            
            
        }
    }
max_profit[actual_length - 1]

}




fn main() {
   println!("The highest profit is: {:?}", highest_profit(vec![7,6,4,3,1]))
}
