use std::{ops::Deref, rc::Rc};

pub fn sub_main() {
   // deref_knowledge();
   test_rc();
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

// 真实的list 也不会这么实现， 用vec<vec <i32>> 来实现更合理。 这么设计删除都不要删除
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

//
enum List2 {
    Cons(i32, Box<List2>),
    Nil,
}

enum List3 {
    Cons(i32, Rc<List3>),
    Nil,
}
pub fn test_cons2() {
    // let a = Rc::new(List2::Cons(5, Rc::new(List2::Cons(10, Rc::new(List2::Nil)))));
    // let b = List2::Cons(3, Rc::clone(&a));
    // let c = List2::Cons(4, Rc::clone(&a));

    // let a = List2::Cons(5, Box::new(List2::Cons(10, Box::new(List2::Nil))));
    // let b = List2::Cons(3, Box::new(a));
    // let c = List2::Cons(4, Box::new(a));

    let a = Rc::new(List3::Cons(
        5,
        Rc::new(List3::Cons(10, Rc::new(List3::Nil))),
    ));
    let b = List3::Cons(3, Rc::clone(&a));
    let c = List3::Cons(4, Rc::clone(&a));

}


fn test_rc() {
    let a = Rc::new(List3::Cons(5, Rc::new(List3::Cons(10, Rc::new(List3::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = List3::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = List3::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}


pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a,T> LimitTracker<'a, T> 
where 
    T: Messager,
{
    pub fn new(messager: & 'a T, max:usize)  -> LimitTracker<'a, T> {
        LimitTracker {
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messager.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messager.send("Warning: You've used up over 75% of your quota!");
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessager {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> MockMessager {
            MockMessager {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messager for MockMessager {
        fn send(&self, msg: &str) {

            //self.sent_messages.borrow_mut().push(String::from(msg));
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

                one_borrow.push(String::from(msg));
                two_borrow.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_quota_message() {
        let mock_messager = MockMessager::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messager.sent_messages.borrow().len(), 1);
        assert_eq!(mock_messager.sent_messages.borrow()[0], "Warning: You've used up over 75% of your quota!");
    }

}