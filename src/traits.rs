use std::marker::PhantomData;

// i have a car
struct Car<T> {
    speed: i32,
    _boost_marker: PhantomData<T>
}

trait Nitrous {
    fn recharge_nitro(&mut self){

    }
}

trait StrongBreaks {}

struct PimpedCar;
impl Nitrous for PimpedCar {}
impl StrongBreaks for PimpedCar{}

struct NormieCar {}
impl Car<PimpedCar>{
    fn new() -> Self {
        Car {speed: 50, _boost_marker: PhantomData}
    }
}

impl Car<NormieCar>{
    fn new() -> Self {
        Car {speed: 20, _boost_marker: PhantomData}
    }
}

// if car is boostable then it should implement boost
impl<T: Nitrous + StrongBreaks> Car<T> {
    fn boost(&mut self) {
        self.speed = self.speed * 2
    }
}


pub(crate) fn main(){
    let mut pimped_car = Car::<PimpedCar>::new();
    pimped_car.boost();

    let mut normie_car = Car::<NormieCar>::new();
    // normie_car.boost(); // rightfully can't call boost()


}