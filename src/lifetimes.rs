
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if(x.len() > y.len()) {x} else {y}
}

pub(crate) fn main(){
    /*
    fails due to lifetime constraints

    let x = "hello";
    let longest_word: &str ;
    {
        let y  = "hi".to_string();
        longest_word= longest(x,&y);
    }

    println!("the longest is {longest_word:?}");*/
}

