
fn add(left: f64, right:f64) -> f64{
    left + right
}
fn another_panic(){
    panic!("hello panic!. this is fine");
}

enum Movement {
    Crawl,
    Walk,
    Run,
    Sprint,
    Fly,
    Swim

}

fn land_movement_speed(movement: Movement) -> Result<i32, String>{

    use Movement::*;

    match movement {
        Crawl => Ok(2),
        Walk => Ok(8),
        Run => Ok(12),
        Sprint => Ok(15),
        Fly => Err("When I fly my feet are off the ground. Fly doesn't have land movement speed".to_string()),
        Swim => Err("When I swim my feet are off the ground. Swim Doesn't have land movement speed".to_string()),

    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}


pub(crate) fn main() {

}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use super::*;


    #[test]
    #[ignore]
    fn super_expensive(){

        thread::sleep(Duration::new(15,0));

        assert_eq!(1,1);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn non_land_movement_returns_error(){
        let speed = land_movement_speed(Movement::Swim);
        assert!(speed.is_err());
    }

    #[test]
    fn land_movement_returns_speed(){
        let speed = land_movement_speed(Movement::Crawl);
        assert_eq!(speed, Ok(2));
    }

    #[test]
    fn will_it_blend(){
        let left = 5.0;
        let right= 7.0;
        let result = 12.0;

        let sum = add(left, right);
        assert_eq!(sum, result, "{} should be the sum of {} + {}",result, left, right);
    }


    #[test]
    #[should_panic(expected = "fine")]
    fn should_panic_on_another_panic(){
        another_panic();
    }

    #[test]
    fn exploration(){
        let sum = add(2.0,2.0);
        assert_eq!(sum, 4.0);
    }

}