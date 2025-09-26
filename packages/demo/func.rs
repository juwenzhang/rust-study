/// 注意这种传递数据的方式，我们的参数是不可变的
fn print_sum(addend1: f64, addend2: f64) -> f64 {
    let sum = addend1 + addend2;
    println!("{} + {} = {}", addend1, addend2, sum);
    sum
}
fn print_diff(subtrahend: f64, minuend: f64) -> f64 {
    let diff = subtrahend - minuend;
    println!("{} - {} = {}", subtrahend, minuend, diff);
    diff
}

// 实现的第一种方式，内部进行接受变化后的数据，但是原本的数据是没有进行变化的
fn print_double(num: f64) -> f64 {
    let double = num * 2.0;
    println!("{} * 2 = {}", num, double);
    double
}
/// 此时的我们的函数就会出现一定的副作用，因为我们的函数内部对参数进行了变化
/// 在函数编程角度来看的话是不好的，尽可能使用纯函数
fn print_double_1(mut num: f64) {
    num *= 2.0;
    println!("{} * 2 = {}", num, num * num);
}

fn foo() -> f64 {
    1.0
}

/// 实现函数返回多个返回值的实现
fn return_many_value() -> (f64, f64) {
    (1.0, 2.0)
}


/// 值传递的实现
/// 这样实现的坏处就是会对参数进行拷贝，占用内存空间，影响性能
fn double_arr_ele(mut a: [i32; 10]) -> [i32; 10] {
    for i in 0..a.len() {
        a[i] *= 2;
    }
    a
}
fn double_arr_ele_ref(a: &mut [i32; 10]) -> &mut [i32; 10] {
    for i in 0..a.len() {
        (*a)[i] *= 2;  // 引用传递的话，需要使用解引用运算符来进行操作
        // 等效于： a[i] = (*a)[i] * 2; | a[i] *= 2
    }
    a
}


// 程序运行的主函数
fn main() {
    let sum = print_sum(1.0, 2.0);
    println!("sum = {}", sum);
    let diff = print_diff(1.0, 2.0);
    println!("diff = {}", diff);
    let double = print_double(1.0);
    println!("double = {}", double);
    print_double_1(1.0);
    let foo = foo();
    println!("foo = {}", foo);
    let (a, b) = return_many_value();
    println!("a = {}, b = {}", a, b);

    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a = double_arr_ele(arr);
    println!("double_arr = {:?}", a);

    let mut arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let _a = double_arr_ele_ref(&mut arr1);
    println!("double_arr = {:?}", arr1);  // 引用函数执行后，数据也是发生了变化了的
}