fn f<T>(num1: T, num2: T) -> T {
    num1 + num2
}

// 泛型枚举的书写
enum Result<SucessCode, FailureCode> {
    Ok(SucessCode),  // 成功的情况
    Err(FailureCode, char),  // 失败的情况
    Uncertain,  // 不确定的情况
}
fn get_result<T>() -> T {
    let mut res = Result::Ok<i32, f64>(12u32);
    match res {
        Result::Ok(v) => v,
        Result::Err(v, c) => v,
        Result::Uncertain => 0,
    }
}


fn main() {
    let a = f::<i32>(1, 2);
    println!("a = {}", a);
}


// 核心的使用细节
enum RESULT<T, E> {
    Ok(T),
    Err(E),
}
fn divide(numerator: i32, denominator: i32) -> RESULT<i32, &'static str> {
    if denominator == 0 {
        RESULT::Err("divide by zero")
    } else {
        RESULT::Ok(numerator / denominator)
    }
}
fn show_divide(numerator: i32, denominator: i32) {
    match divide(numerator, denominator) {
        RESULT::Ok(v) => println!("{}", v),
        RESULT::Err(v) => println!("{}", v),
        _ => println!("unknown error"),
    }
}