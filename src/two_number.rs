use core::num;
use std::collections::HashMap;

#[test]
fn main() {
    let nums = vec![3, 3];
    let target = 6;
    let mut map = HashMap::new();
    for (idx, item) in nums.iter().enumerate() {
        map.insert(item, idx);
    }
    let mut result = Vec::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        let complement_option = map.get(&complement);
        if let Some(&complement_index) = complement_option {
            if complement_index != i {
                result.push(i as i32);
                result.push(map[&complement] as i32);
                break;
            }
        }
    }
    println!("{:?}", result);
}
