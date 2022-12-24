impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut cnt:i8 = 2;
        let mut i:i32 = 2;
        while i * i < n {
            if n % i == 0 {
                cnt += 2;
            }
            i += 1
        }
        if i * i == n {
            cnt += 1
        }

        return (cnt == 3)
    }
}
