use std::any::type_name_of_val;
use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;
use crate::smart_pointers::List::{Cons, Nil};

#[derive(Debug)]
struct DropNotifier(String);

impl Drop for DropNotifier {
    fn drop(&mut self) {
        println!("{:?} is dropped", self.0);
    }
}


#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}



impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>>{
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
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

struct Thingy {
    stuff: String,
}

struct PrintMessenger {}

impl Messenger for PrintMessenger {
    fn send(&self, message: &str) {
        println!("{message}");
    }
}

pub trait Messenger {
    fn send(&self, message: &str);
}

pub struct LimitTracker <'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T>{
        LimitTracker::<'a, T> {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize){
        self.value = value;

        let percentage_of_max = value as f64 / self.max as f64;

        match percentage_of_max {
            percentage if percentage >= 1.0 => {
                self.messenger.send("you've reached your limit!")
            }
            percentage if percentage >= 0.75 => {
                self.messenger.send("you've almost reached your limit (10% left)")
            }
            percentage if percentage >= 0.75 => {
                self.messenger.send("you soon reach your limit (25% left)")
            }
            _ => ()
        }

    }
}

pub(crate) fn main(){


    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    //
    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());
    //
    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    //
    // println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // println!("b initial rc count = {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());
    //
    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }
    //
    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));
    //
    // println!("a next item = {:?}", a.tail());


    // let print_messenger = PrintMessenger {};
    // let mut tracker = LimitTracker::new(&print_messenger, 100);
    //
    // tracker.set_value(90);


    // let mut x = 5;
    // let y = &mut x;
    //
    // let a = RefCell::new(Thingy {stuff: "Hello".to_string()});
    // let b = a.borrow_mut();
    // b


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


    // let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    // let b = List::Cons(3, a.clone());
    // let c = List::Cons(4, a.clone());



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

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger{
        messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self{
            Self {
                messages: RefCell::new(vec!()),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.messages.borrow_mut().push(message.to_string());
        }
    }

    #[test]
    fn creates_message_if_value_reaches_75_percent(){
        let mock_messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock_messenger, 100);

        tracker.set_value(75);

        assert_eq!(mock_messenger.messages.borrow().len(), 1);
    }

}