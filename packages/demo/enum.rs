fn demo_1() {
    #[warn(dead_code)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let d = Direction::Up;
    println!("{:?}", match d {
        Direction::Up => "向上",
        Direction::Down => "向下",
        _ => "其他",
    });
}

#[warn(unused_imports)]
fn main() {
    // 实现定义枚举
    enum Result {
        Sucess(f64),
        Error(u16, char),
        Uncertainty,
    }

    // 实现使用枚举
    let mut outcome = Result::Sucess(100.0);
    match outcome {
        Result::Sucess(v) => println!("成功，结果为：{}", v),
        Result::Error(code, ch) => println!("错误，错误码为：{}，错误信息为：{}", code, ch),
        Result::Uncertainty => println!("不确定性"),
    }

    outcome = Result::Error(1200, 'X');
    match outcome {
        Result::Sucess(v) => println!("成功，结果为：{}", v),
        Result::Error(code, ch) => println!("错误，错误码为：{}，错误信息为：{}", code, ch),
        Result::Uncertainty => println!("不确定性"),
    }

    outcome = Result::Uncertainty;
    match outcome {
        Result::Sucess(v) => println!("成功，结果为：{}", v),
        Result::Error(code, ch) => println!("错误，错误码为：{}，错误信息为：{}", code, ch),
        Result::Uncertainty => println!("不确定性"),
    }

    demo_1();
}

// 输出为： rustc enum.rs -o output && ./output  : 默认的就是输出 enum 文件如果不指定 -o 的话