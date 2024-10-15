// O(log(b))
pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let reminder = a % b;
        a = b;
        b = reminder
    }
    a
}