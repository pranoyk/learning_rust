use std::collections::HashMap;


fn median(list_of_numbers: &mut Vec<i32>) -> f32 {
    let len = list_of_numbers.len();
    if len % 2 == 0 {
        return (list_of_numbers[len/2 - 1] + list_of_numbers[len/2]) as f32 / 2.0
    }
    list_of_numbers[len/2] as f32
}

fn mode(list_of_numbers: &mut Vec<i32>) -> i32 {
    let mut hash_map = HashMap::new();
    for &num in list_of_numbers.iter() {
        let c = hash_map.entry(num).or_insert(0);
        *c += 1
    }
    let mut mode = list_of_numbers[0];
    let mut max_freq = 0;
    for (&num, &count) in hash_map.iter() {
        if count > max_freq {
            max_freq = count;
            mode = num;
        }
    }
    mode
}

fn main() {
    let mut list_of_numbers = vec![4, 5, 1, 3, 3, 2, 2, 3];
    let median = median(&mut list_of_numbers);
    println!("Median: {}", median);
    let mode = mode(&mut list_of_numbers);
    println!("Mode: {}", mode);
}