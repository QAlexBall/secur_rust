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
