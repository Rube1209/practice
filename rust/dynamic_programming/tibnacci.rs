fn Tib(n: i32) -> i32 {
    if n == 0 {
        return 0; // T(0) = 0
    }
    if n == 1 || n == 2 {
        return 1; // T(1) = 1, T(2) = 1
    }
    
    let mut a = 0; // T(0)
    let mut b = 1; // T(1)
    let mut c = 1; // T(2)
    let mut d = 0; // T(n)

    // 从 i = 3 开始计算 T(n)
    for i in 3..=n {
        d = a + b + c; // 计算 T(i) = T(i-1) + T(i-2) + T(i-3)
        a = b; // 更新 T(i-3)
        b = c; // 更新 T(i-2)
        c = d; // 更新 T(i-1)
    }
    
    return c; // 返回 T(n)
}

fn main() {
    let n = 4; // 示例: 计算第 4 个泰波那契数
    println!("T({}) = {}", n, Tib(n));
}
