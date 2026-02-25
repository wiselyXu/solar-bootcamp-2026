use std::ops::Deref;

pub fn sub_main() {
    deref_knowledge();
}

fn deref_knowledge() {
    //deref_part01_common_ref();
    //defer_part02_box();
    defer_part03_custom_box();
}

fn deref_part01_common_ref() {
    println!("引用的通用表达");
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(&x, y);
    assert_eq!(x, *y);
}

fn defer_part02_box() {
    println!("Box<T> 智能指针, 它允许在堆上分配数据，并且提供了类似于引用的语法来访问这些数据。");
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    //assert_eq!(&x, y);
    assert_eq!(x, *y);
}

fn defer_part03_custom_box() {
    println!("Box<T> 智能指针, 它允许在堆上分配数据，并且提供了类似于引用的语法来访问这些数据。");
    let x = 5;
    let y = MyBox::new(x);
    println!("y: {}", *y);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    //assert_eq!(&x, y);
    assert_eq!(x, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn test_cons() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
}
