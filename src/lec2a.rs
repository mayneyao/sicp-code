// 高阶过程

// 从 a 到 b 的和
fn sum_int(a: i32, b: i32) -> i32 {
    if a > b {
        0
    } else {
        return a + sum_int(a + 1, b);
    }
}

// 从 a 到 b 的平方和
fn sum_sq(a: i32, b: i32) -> i32 {
    if a > b {
        0
    } else {
        return a * a + sum_sq(a + 1, b);
    }
}

// dry
// 函数做参数
// 求和通项
fn sum(term: fn(i32) -> i32, a: i32, next: fn(i32) -> i32, b: i32) -> i32 {
    if a > b {
        0
    } else {
        return term(a) + sum(term, next(a), next, b);
    }
}

//
fn new_sum_int(a: i32, b: i32) -> i32 {
    let term = |x| x;
    let next = |x| x + 1;
    return sum(term, a, next, b);
}

fn new_sum_sq(a: i32, b: i32) -> i32 {
    let term = |x| x * x;
    let next = |x| x + 1;
    return sum(term, a, next, b);
}

fn main() {
    println!("{}", sum_int(1, 10));
    println!("{}", new_sum_int(1, 10));
    println!("{}", sum_sq(1, 10));
    println!("{}", new_sum_sq(1, 10));
}
