use core::num;

fn main() {
    println!("Hello, world!");
    three_sum(vec![1, -1, 0]);
}
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut i = 0;
    let mut result: Vec<Vec<i32>> = Vec::new();

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let mut l = i + 1;
        let mut r = nums.len() - 1;
        while l < r {
            let sum = nums[i] + nums[l] + nums[r];
            if sum == 0 {
                result.push(vec![nums[i], nums[l], nums[r]]);
                l += 1;
                while nums[l] == nums[l - 1] && l < r {
                    l += 1;
                }
            } else if sum > 0 {
                r -= 1;
            } else if sum < 0 {
                l += 1;
            }
        }
    }
    result
}
