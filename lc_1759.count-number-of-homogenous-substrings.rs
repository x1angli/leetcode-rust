// Note: the default leetcode interpretor doesn't support itertools, so plz run the code use your own Rust Env / or a 3rd-party Playground

use itertools::Itertools;

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {

        let mut result:u64 = 0;
        s.chars()
            .into_grouping_map_by(|&x| x)
            .fold(0, |acc, _key, _value| acc + 1)
            .into_iter()
            .for_each(|(_, v)| result += ((v*v+1)/2) as u64);

        return (result % 1000000007) as i32;
    }
}
