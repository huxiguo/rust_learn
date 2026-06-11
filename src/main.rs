// 主函数
fn main() {
    println!("Hello, world!");
    add(2);

    let number = 3;

    if number < 5 {
        println!("number is less than 5");
    }
}

fn add(a: i32) -> i32 {
    println!("a is {a}");
    a + 1
}
