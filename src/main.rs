fn ack(m: i32, n: i32) -> i32 {
    let ans: i32;
    if m == 0 {
        ans = n + 1;
    } else if n == 0 {
        ans = ack(m - 1, 1);
    } else {
        ans = ack(m - 1, ack(m, n - 1));
    }

    return ans;
}

fn main() {
    let limit: i32 = 6;

    for i in 0..limit {
        for j in 0..limit {
            println!("ackermann ({}, {}) is {}", i, j, ack(i, j));
        }
    }
}
