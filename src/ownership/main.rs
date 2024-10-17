// File       : main.rs
// Time       ：2024/9/30 14:21
// Author     ：sandwich
// version    ：V1.0
// Description：

fn get_string(s: String) {
    println!("{s}");
}

#[test]
fn test_box() {
    let point = Box::new((0.625, 0.5)); // 分配point
    let label = format!("{:?}", point); // 分配label
    println!("{label}");
} // point label dropped

#[test]
fn test_ownership_tree() {
    struct Person {
        name: String,
        birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
    composers.push(Person { name: "Dowland".to_string(), birth: 1563 });
    composers.push(Person { name: "Lully".to_string(), birth: 1632 });
    for composer in &composers {
        println!("{} bron:{}", composer.name, composer.birth);
    }
}

#[test]
fn test_move() {
    let mut s = "Govinda".to_string();
    let _t = s;
    // 注意变量s回到未初始化状态，而不是清除了
    s = "Siddhartha".to_string();
    println!("{s}");
}

#[test]
fn test_move_with_index() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    /* 无法直接转移向量元素的所有权，考虑借用或用其他转移元素所有权的方法
    let thrid = v[2];
    let fifth = v[4];
     */
    // 转移所有权的方法
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");
    // 取值，用最后一个元素填充
    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    // 其他值交换取出的值
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
    assert_eq!(v, vec!["101", "104", "substitute"]);

    let v = vec!["liberté".to_string(),
                 "égalité".to_string(), "fraternité".to_string()];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
}

#[test]
fn test_copy() {
    #[derive(Copy, Clone)]
    struct Label {
        number: u32,
    }
    fn print(label: Label) {
        println!("STAMP: {}", label.number);
    }
    let l = Label { number: 3 };
    print(l); // moved here, use #derive change to Copy
    println!("My label number is {}", l.number);
}

#[test]
fn test_rc() {
    use std::rc::Rc;
    let s = Rc::new("shirataki".to_string());
    let t = s.clone();
    let u = s.clone();
    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
    // rc值不可修改 s.push('!');
}

fn main() {
    // 基本数据类型，array，tuple赋值时都是执行copy特性
    let mut arr = [1, 3, 5];
    let mut arr_1 = arr;
    arr[0] = 3;
    arr_1[0] = 99;
    println!("after change arr: {:?}", arr);
    println!("after change arr_1: {:?}", arr_1);

    for item in &arr_1 {
        println!("element: {item}");
    }

    // 复杂类型赋值执行move操作
    let s = String::from("hello");
    let s1 = s; // move
    // println!("{s}"); // borrow of moved value
    println!("{s1}");

    get_string(s1);
    // println!("{s1}"); // s1 move to fn get_string
}