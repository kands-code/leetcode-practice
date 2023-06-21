use std::collections::HashMap;

fn main() {
    let v = vec![3, 2, 4];
    println!("res = {:?}", two_sum(v, 6));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sum_table = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        if sum_table.contains_key(&(target - v)) {
            if let Some(&idx) = sum_table.get(&(target - v)) {
                return vec![i as i32, idx];
            }
        }
        sum_table.insert(v, i as i32);
    }
    panic!("out of boundary!");
}
