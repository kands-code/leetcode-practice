fn main() {
    let v = vec![1, 1, 4, 2, 1, 3];
    println!("{:?}", height_checker(v));
}

pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut expected = heights.clone();
    expected.sort();
    expected
        .iter()
        .enumerate()
        .filter(|(i, v)| -> bool {
            if let Some(e) = heights.get(*i) {
                e != *v
            } else {
                false
            }
        })
        .count() as i32
}
