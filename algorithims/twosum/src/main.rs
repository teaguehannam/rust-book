use std::collections::HashMap;
use std::convert::TryInto;

// Given an array of integers nums and an integer target,
// return indices of the two numbers such that they add up to target.

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_vec: HashMap<i32, i32> = HashMap::new();

    for (i, item) in nums.iter().enumerate() {
        // find matching number (target num - current num)
        match num_vec.get(&(target - item)) {
            Some(value) => {
                // value exists, return indices
                return vec![i.try_into().unwrap(), *value];
            }
            None => {
                // didn't find (target num - current num)
                // add for (target num - future num) possible match 
                num_vec.insert(*item, i.try_into().unwrap());
            }
        }
    }

    vec![]
}

fn main() {
    let nums = vec![2, 3, 5, 7, 11, 16];
    let solution = two_sum(nums, 27);
    println!("{:?}", solution);
}
