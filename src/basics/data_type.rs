pub fn data_type() {
    // 整型
    println!("========== 有符号 i8, i16, i32, i64, i128, isize ==========");
    let max_i8: i8 = 127;
    println!("max_i8 is {max_i8}");
    let min_i8: i8 = -128;
    println!("min_i8 is {min_i8}");
    let max_i16: i16 = i16::MAX;
    println!("max_i16 is {max_i16}");
    let min_i16: i16 = i16::MIN;
    println!("min_i16 is {min_i16}");
    let max_i32: i32 = i32::MAX;
    println!("max_i32 is {max_i32}");
    let min_i32: i32 = i32::MIN;
    println!("min_i32 is {min_i32}");
    let max_i64: i64 = i64::MAX;
    println!("max_i64 is {max_i64}");
    let min_i64: i64 = i64::MIN;
    println!("min_i64 is {min_i64}");
    let max_i128: i128 = i128::MAX;
    println!("max_i128 is {max_i128}");
    let min_i128: i128 = i128::MIN;
    println!("min_i128 is {min_i128}");
    let max_isize: isize = isize::MAX;
    println!("max_isize is {max_isize}");
    let min_isize: isize = isize::MIN;
    println!("min_isize is {min_isize}");

    println!("========== 无符号 u8, u16, u32, u64, u128, usize ==========");
    let max_u8: u8 = u8::MAX;
    println!("max_u8 is {max_u8}");
    let min_u8: u8 = u8::MIN;
    println!("min_u8 is {min_u8}");
    let max_u16: u16 = u16::MAX;
    println!("max_u16 is {max_u16}");
    let min_u16: u16 = u16::MIN;
    println!("min_u16 is {min_u16}");
    let max_u32: u32 = u32::MAX;
    println!("max_u32 is {max_u32}");
    let min_u32: u32 = u32::MIN;
    println!("min_u32 is {min_u32}");
    let max_u64: u64 = u64::MAX;
    println!("max_u64 is {max_u64}");
    let min_u64: u64 = u64::MIN;
    println!("min_u64 is {min_u64}");
    let max_u128: u128 = u128::MAX;
    println!("max_u128 is {max_u128}");
    let min_u128: u128 = u128::MIN;
    println!("min_u128 is {min_u128}");
    let max_usize: usize = usize::MAX;
    println!("max_usize is {max_usize}");
    let min_usize: usize = usize::MIN;
    println!("min_usize is {min_usize}");
    println!("========== 浮点数 f32, f64 ==========");

    let max_f32: f32 = f32::MAX;
    println!("max_f32 is {max_f32}");
    let min_f32: f32 = f32::MIN;
    println!("min_f32 is {min_f32}");
    let max_f64: f64 = f64::MAX;
    println!("max_f64 is {max_f64}");
    let min_f64: f64 = f64::MIN;
    println!("min_f64 is {min_f64}");

    println!("========== 布尔值 bool ==========");
    let true_value: bool = true;
    println!("true_value is {true_value}");
    let false_value: bool = false;
    println!("false_value is {false_value}");

    println!("========== 字符串 str ==========");
    let string_value: &str = "Hello, world!";
    println!("string_value is {string_value}");

    let char_value: char = 'ℤ';
    println!("char_value is {char_value}");


    println!("========== 复合数据类型 ==========");

    println!("========== 元组 tuple ==========");

    let tuple_value: (i32, f64, bool) = (1, 2.0, true);
    println!("tuple_value is {:?}", tuple_value);

    // 解构
    let (x, y, z) = tuple_value;
    println!("x is {x}, y is {y}, z is {z}");

    // 访问元组中的元素
    println!("the first element is {}", tuple_value.0);
    println!("the second element is {}", tuple_value.1);
    println!("the third element is {}", tuple_value.2);

    // 数组
    println!("========== 数组 array ==========");

    let array_value: [i32; 3] = [1, 2, 3];
    println!("array_value is {:?}", array_value);

    // 访问数组中的元素
    println!("the first element is {}", array_value[0]);
    println!("the second element is {}", array_value[1]);
    println!("the third element is {}", array_value[2]);
    
}