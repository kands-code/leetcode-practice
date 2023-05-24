fn main() {
    println!("{:?}", is_palindrome(123));
}

pub fn is_palindrome(x: i32) -> bool {
    let x_str = x.to_string();
    x_str.chars().rev().collect::<String>() == x_str
}
