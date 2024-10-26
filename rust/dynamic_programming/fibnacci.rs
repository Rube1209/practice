fn safe_fibonacci(n: i32) -> u32 {
    if n < 0 {
        panic!("Fibonacci is not defined for negative numbers");
    }
    let n = n as u32; // 显式转换
    if n == 0 || n == 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b // 返回 u32
}

fn main() {
    let n = 10; // 计算第 10 个 Fibonacci 数
    println!("Fibonacci({}) = {}", n, safe_fibonacci(n));
}
