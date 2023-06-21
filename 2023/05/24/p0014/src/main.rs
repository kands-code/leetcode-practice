fn main() {
    println!("{:?}", longest_common_prefix(vec!["".to_owned()]));
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::new();
    let min_str = strs
        .iter()
        .min_by(|s1, s2| s1.len().partial_cmp(&s2.len()).unwrap());
    for c in min_str.unwrap().chars() {
        let backup = prefix.clone();
        prefix.push(c);
        if !strs.iter().all(|s| s.starts_with(&prefix)) {
            return backup.clone();
        }
    }
    prefix
}
