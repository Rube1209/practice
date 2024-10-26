fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 2;
    }

    let mut first: i32 = 1;
    let mut second: i32 = 2;
    let mut ways: i32 = 0;

    for i in 3..=n {
        ways = first + second;
        first = second;
        second = ways;
    }
    
    return ways;
}

