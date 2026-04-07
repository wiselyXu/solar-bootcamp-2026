pub fn sub_main() {
    //  println!(" from book->twenty ->unsafe_mod");
        raw_pointer();
}

fn raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!(
        "we can use r1 and r2 in raw pointer, but need unsafe block, both point to a same memory address"
    );
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
