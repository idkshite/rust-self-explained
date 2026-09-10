use std::any::type_name_of_val;
use std::cell::Cell;
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil
}

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(val: T) -> MyBox<T> {
        Self(val)
    }
}

impl<T> Deref for MyBox<T> {

    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub(crate) fn main(){


    let num = 5;
    let num_ref = MyBox::new(num);

    hello(&MyBox::new("hello".to_string()));

    assert_eq!(5, *num_ref);



    //
    // let num_ref;
    // {
    //     let num = 5;
    //     num_ref = Box::new(num);
    // }

    // let boxxed = Box::new(num);
    //
    // println!("box {boxxed:?}, {:?}", type_name_of_val(&boxxed));
    // println!("deref box {:?}, {:?}", *boxxed, type_name_of_val(&*boxxed));

    // let hey = *num_ref;

    // assert_eq!(5, 5);
    // assert_eq!(*num_ref, num);


    // Cell::new(2);
    //
    // let conslist = List::Cons(1,Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

}