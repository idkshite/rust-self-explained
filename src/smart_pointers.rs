use std::any::type_name_of_val;
use std::cell::Cell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
struct DropNotifier(String);

impl Drop for DropNotifier {
    fn drop(&mut self) {
        println!("{:?} is dropped", self.0);
    }
}


enum List {
    Cons(i32, Rc<List>),
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

fn consume_drop_notifier(dn: DropNotifier){
    let _ = format!("hey {}", dn.0);
}

pub(crate) fn main(){

   //  let dn1 = DropNotifier("peter1".to_string());
   //  let dn2 = DropNotifier("peter2".to_string());
   //
   // //  consume_drop_notifier(dn1);
   //
   //  println!("Drop Notifiers Created");
   //
   //  let b = &"bubba".to_string();
   //
   //  let a = Rc::new(Box::new(List::Cons(5, Rc::new(Box::new(List::Cons(10, Rc::new(Box::new(List::Nil))))))));
   //  println!("Count {:?}", Rc::strong_count(&a));
   //  let b = List::Cons(3, Rc::clone(&a));
   //  println!("Count {:?}", Rc::strong_count(&a));
   //
   //  let c = List::Cons(4, Rc::clone(&a));
   //  println!("Count {:?}", Rc::strong_count(&a));


    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let b = List::Cons(3, a.clone());
    let c = List::Cons(4, a.clone());



    // let num = 5;
    // let num_ref = MyBox::new(num);
    //
    // hello(&MyBox::new("hello".to_string()));
    //
    // assert_eq!(5, *num_ref);

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