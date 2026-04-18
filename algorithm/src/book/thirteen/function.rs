pub fn sub_main(){
   // println!("this is from book/thirteen/funciton");
  // assign_shirt_color();
  test_closure();
}


#[derive(Debug,Copy,Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>)  -> ShirtColor {
         user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_num = 0;
        let mut blue_num = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => blue_num +=1,
                ShirtColor::Red => red_num +=1,
            }
        }

        if red_num > blue_num {
            ShirtColor::Red
        }else {
            ShirtColor::Blue
        }

    }
    
}

fn assign_shirt_color() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue,ShirtColor::Blue,ShirtColor::Red,]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!("the user with preference {:?} gets {:?}" , user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("the user with preference {:?} gets {:?}" , user_pref2, giveaway2);


}

fn test_closure(){
    let mut list = vec![1,3,4,5,6];
    println!("before defining closure: {list:?}");
 //  let only_borrow = || println!("from closure : {list:?}");
    let mut add_one = ||  {
        for val in list.iter_mut() {
            *val +=1;
        }
    };

    let list2 = vec![7,5,8,9,10];
    let plus_one_vec = || list2.iter().map(|&i| i+1).collect::<Vec<_>>();
    let list3 = plus_one_vec();
    println!("list2 after closure: {list2:?}");
    println!("list3 after closure: {list3:?}");

   // println!("before calling closure: {list:?}");
  //  only_borrow();
    // println!("after calling closure: {list:?}");

    add_one();
    println!("after calling mut closure: {list:?}");
}
