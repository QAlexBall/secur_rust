pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    if nums.len() < 3 {
        return vec![];
    }

    let mut nums = nums.clone();
    nums.sort();

    let mut i = 0;
    while i != nums.len() - 2 || nums[i] < 0 {
        if i > 0 && nums[i] == nums[i - 1] {
            i += 1;
            continue;
        }
        let mut l = i + 1;
        let mut r = nums.len() - 1;
        while l < r {
            let sum = nums[i] + nums[l] + nums[r];
            if sum == 0 {
                res.push(vec![nums[i], nums[l], nums[r]]);
                while l < r && nums[l] == nums[l + 1] {
                    l += 1;
                }
                while l < r && nums[r] == nums[r - 1] {
                    r -= 1;
                }
                l += 1;
                r -= 1;
            } else if sum < 0 {
                l += 1;
            } else {
                r -= 1;
            }
        }
        i += 1;
    }
    res
}

use super::solution::SExcutable;

#[derive(Debug, Clone)]
pub struct ThreeSum {
    s_name: String,
    arg_num: String,
    s_args: String,
    args: Vec<i32>,
}

impl ThreeSum {
    pub fn new(s_name: String, arg_num: String, s_args: String) -> Self {
        let args = vec![-4, -5, 3, 1, 0, 2, -1];
        Self {
            s_name,
            arg_num,
            s_args,
            args,
        }
    }

    pub fn detail(&self) {
        println!(
            "@s_name {}, @arg_num {}, @s_args {}",
            self.s_name, self.arg_num, self.s_args
        );
    }
}

impl SExcutable<Vec<Vec<i32>>> for ThreeSum {
    fn exec(&self) -> Vec<Vec<i32>> {
        self.detail();
        three_sum(self.args.clone())
    }
}

pub fn display_result(item: &impl SExcutable<Vec<Vec<i32>>>) {
    println!("{:?}", item.exec());
}
