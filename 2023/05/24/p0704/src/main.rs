fn main() {
    let v = vec![-1, 0];
    println!("return: {:?}", search(v, -1));
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut lb = 0;
    let mut ub = nums.len();
    let mut mid = (lb + ub) / 2;
    while lb < ub {
        if let Some(val) = nums.get(mid) {
            if val > &target {
                ub = mid;
            } else if val < &target {
                lb = mid + 1;
            } else {
                return mid as i32;
            }
            mid = (lb + ub) / 2;
        } else {
            panic!("out of boundary!");
        }
    }
    return -1;
}
