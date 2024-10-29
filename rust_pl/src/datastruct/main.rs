/*
 * File              : main.rs
 * Author            : sandwich
 * Date              : 2024-02-23 09:07:49
 * Last Modified Date: 2024-08-02 14:04:07
 * Last Modified By  : sandwich
 */

#[test]
fn test_char_and_u8() {
    assert_eq!(b'A', 65_u8);
    assert_eq!(b'\\', 92_u8);
    assert_eq!(b'\\', b'\x5c');
}

#[test]
fn test_numeric_type() {
    // 类型转换 截短 数字
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(-1i8 as u8, 255u8);
    // 类型自带方法
    assert_eq!(2u16.pow(4), 16);
    // 使用时必须指定类型
    assert_eq!((-4i32).abs(), 4);
    assert_eq!(-1.01f64.floor(), -1.0); // 注意没有括号 1.01f64.floor()
}

#[test]
fn test_tuple() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}

#[test]
fn test_array() {
    // array 同一类型、固定序列
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    // 批量生成
    let mut sieve = [true; 10000];
    // 对非质数索引的值进行修改
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);
    assert!(sieve[17]);

    // 数组内置方法
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

#[test]
fn test_vector() {
    // 宏创建
    let v = vec![2, 3, 5, 7];
    // reduce
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);
    // 初始化空vec
    let mut v = Vec::new();
    v.push("step");
    v.push("on");
    v.push("no");
    v.push("pets");
    assert_eq!(v, vec!["step", "on", "no", "pets"]);
    // 基于迭代器初始化向量
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    // 反转
    let mut v = vec!["a man", "a plan", "a canal", "panama"];
    v.reverse();
    // Vec目前与array的表现一致了
    assert_eq!(v, vec!["panama", "a canal", "a plan", "a man"]);

    // 预分配大小
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);
    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);
    v.push(3);
    assert_eq!(v.len(), 3);
    // 不一定可以通过测试
    assert_eq!(v.capacity(), 4);
    let mut v = vec![10, 20, 30, 40, 50];
    // insert
    v.insert(3, 35);
    assert_eq!(v, vec![10, 20, 30, 35, 40, 50]);
    // remove
    v.remove(1);
    assert_eq!(v, vec![10, 30, 35, 40, 50]);

    // pop -> Option<T>
    let mut v = vec!["carmen", "miranda"];
    assert_eq!(v.pop(), Some("miranda"));
    assert_eq!(v.pop(), Some("carmen"));
    assert_eq!(v.pop(), None);
}

#[test]
/// # test string
///
/// show the details
fn test_str() {
    println!("this is
    a hola file");
    println!("this is a \
    hola file");
    println!(r"c:\data\test.rust");
    println!(r###"start:
    test the spaces in rule string
    show me!
    "###);
    // format
    assert_eq!(format!("{}°{:02}′{:02}″", 24, 5, 23), "24°05′23″");
    // 字符串数组、切片、向量的拼接
    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");
    for word in bits {
        assert!(word.starts_with('v'));
    }
}

fn main() {
    // array 声明与初始化
    let arr1: [i32; 5] = [1, 3, 5, 7, 9];
    println!("{arr1:?}");
    let arr2 = [3; 5];
    println!("{arr2:?}");
    // 索引访问
    println!("arr element: {}", arr1[2]);

    let tup = (1, "str");
    println!("{tup:?}");
    // 访问元组
    println!("tup element: {}", tup.1);
    // 元组模式匹配
    let (x, s) = tup;
    println!("匹配结果: {}, {}", x, s);

    // char 与 u8
    let x = 'A';
    println!("{}", x as u8);

    // 命令行 -> 收集vector -> 遍历
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l,
                 // 基于表达式的rust
                 if l.len() % 2 == 0 {
                     "functional"
                 } else {
                     "imperative"
                 });
    }
}