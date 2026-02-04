/**
 * 变量和可变形
 */
pub fn variables() {
    // 不可变变量
    let x = 5;
    println!("x is {x}");
    // 会报错，因为x是不可变变量
    // x = 6;
    // println!("x is {x}");

    // 可变变量
    // 使用mut关键字声明可变变量
    let mut y = 6;
    println!("y is {y}");
    y = 7;
    println!("y is {y}");

    // 常量
    // 使用const关键字声明常量
    const ONE_DAY_IN_MINUTES: u32 = 60 * 24;
    println!("ONE_DAY_IN_MINUTES is {ONE_DAY_IN_MINUTES}");

    // 遮蔽 Shadowing
    let z = 8;
    println!("z is {z}");
    // 重新声明变量z，会遮蔽之前的z
    let z = z + 1;
    println!("z is {z}"); // 9

    {
        // 作用域内的变量z，会遮蔽外面的变量z
        let z = z + 1;
        println!("z is {z}"); // 10
    }
    println!("z is {z}"); // 9
}
