pub(crate) fn main(){

    let  x: i32 = 12;

    let mut name = "peter".to_string();

    // owned(name);

    // exclusive_ref(&mut name);

    // no_star(&mut name);

    // println!("{name}");

    // let mut numbers = [10,20,30,40,50];
    // println!("{numbers:?}");
    // zero_alternating(&mut numbers);
    // println!("{numbers:?}")

    println!("{:?}", owned(name))

}


struct Person {

}

trait Nameable {
    fn name(title: &mut String);
}


fn owned(mut name:String) -> String{
    println!("{name}");
    name = "martha".to_string();
    println!("{name}");
    name
}

fn exclusive_ref(mut name:&mut String){
    println!("{name}");
    *name = "martha".to_string();
    println!("{name}");
}

fn no_star(name: &mut String) {
    name.push_str(" jr");
    name.make_ascii_uppercase();
    let old = std::mem::replace(name, "martha".to_string());
    println!("took {:?}, now {:?}", old, name);
}


fn zero_alternating(mut xs: &mut [i32]) {
    while let Some((first, rest)) = xs.split_first_mut() {
        *first = 0;                                        // &mut: reach the caller
        xs = if rest.is_empty() { rest } else { &mut rest[1..] };  // mut: re-aim myself
    }
}


fn funky(&mut val: &mut i32){
    
}

fn hello(name:&mut String){
    println!("{name}");
    *name = "martha".to_string();
    println!("{name}");
}

