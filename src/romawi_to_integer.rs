use std::collections::HashMap;
fn main() {
    let input = "LVIII";
    let mut hashmap = HashMap::new();
    hashmap.insert("I", 1);
    hashmap.insert("V", 5);
    hashmap.insert("X", 10);
    hashmap.insert("L", 50);
    hashmap.insert("C", 100);
    hashmap.insert("D", 500);
    hashmap.insert("M", 1000);

    let mut checking = "";
    let mut result = 0;
    let mut i = 0;
    
    while i < input.len() {
        if i + 1 < input.len() {
            checking = &input[i..=i + 1];
            let first_chars = checking.chars().nth(0).unwrap();
            let second_chars = checking.chars().nth(1).unwrap();

            if first_chars == 'I' && second_chars == 'V' {
                result += 4;
                i += 2;
            } else if first_chars == 'I' && second_chars == 'X' {
                result += 9;
                i += 2;
            } else if first_chars == 'X' && second_chars == 'L' {
                result += 40;
                i += 2;
            } else if first_chars == 'X' && second_chars == 'C' {
                result += 90;
                i += 2;
            } else if first_chars == 'C' && second_chars == 'D' {
                result += 400;
                i += 2;
            } else if first_chars == 'C' && second_chars == 'M' {
                result += 900;
                i += 2;
            } else {
                checking = &input[i..=i];
                let first_chars = checking.chars().nth(0).unwrap();
                let value = hashmap.get(first_chars.to_string().as_str());
                let get_value_from_hash = value.unwrap();
                result += get_value_from_hash;
                i += 1;
            }
        } else {
            checking = &input[i..=i];
            let first_chars = checking.chars().nth(0).unwrap();
            let value = hashmap.get(first_chars.to_string().as_str());
            let get_value_from_hash = value.unwrap();
            result += get_value_from_hash;
            i += 1;
        }
    }
    println!("{}", result);
}
