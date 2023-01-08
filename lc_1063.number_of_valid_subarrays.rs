impl Solution {
    pub fn valid_subarrays(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut stack = vec![];

        for num in nums {
            while let Some(&head_val) = stack.last() {
                if head_val > num { 
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(num);
            ans += stack.len();
        }

        ans as i32
    }
}
