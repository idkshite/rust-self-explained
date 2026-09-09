#[derive(Debug, Eq, PartialEq)]
enum NumberOrString{
    S(String),
    N(i32)
}

fn house_number(number: String) -> NumberOrString{
    let has_letter = number.chars().any(|c| {
        c.is_alphabetic()
    });

    if(has_letter) {
        return NumberOrString::S(number)
    }

    NumberOrString::N(number.parse::<i32>().unwrap())



}


pub(crate) fn main(){

    // TODO: Write tests for house_number()

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn string_house_number() {
        assert_eq!(house_number("3A".to_string()), NumberOrString::S("3A".to_string()));
    }

    #[test]
    fn numeric_house_number(){
        assert_eq!(house_number("3".to_string()), NumberOrString::N(3));
    }

}


