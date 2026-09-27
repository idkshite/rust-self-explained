use std::fmt::{Display, Formatter};

#[derive(Debug, Clone,Copy)]
enum Ripeness {
    HardLikeAStone,
    SqueezeMe,
    MaybeGreen,
    Green,
    Perfection,
    Foul,
}

#[derive(Debug, Clone, Copy)]
struct Avocado {
    ripeness: Ripeness,
    price: f32,
}

impl Display for Avocado {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {})", self.ripeness, self.price)
    }
}

impl Avocado {
    fn name() -> String {
        "Avocado".to_string()
    }

    fn raise_price(&mut self) {
        self.price = self.price * 2.0
    }

    fn set_price(&mut self, price: f32) {
        self.price = price
    }

    fn formatted_price(&self) -> String {
        format!("{:?} EUR", self.price.to_string())
    }
}

pub(crate) fn closures_main() -> () {
    let list = vec![1, 2, 3];
    let some_string = "boogy woogy".to_string();
    let some_number = 23;

    let mut avocado = Avocado {
        ripeness: Ripeness::Green,
        price: 1.30,
    };

    println!("What a list: {list:?}");
    println!("A great avocado: {avocado:?}");
    println!("A great avocado: {:?}", avocado);
    println!("A great avocado: {:?}", avocado);
    println!("A great avocado: {}", avocado);

    avocado.raise_price();
    avocado.raise_price();

    println!("A great avocado, but pricey: {avocado}");
    let mut mess_with_price = || avocado.price = rand::random::<f32>() * 500.0;
    // let mut mess_with_price = || avocado.set_price(rand::random());
    mess_with_price();

    println!(
        "A great avocado: {} and a number {:?} with a {:?} string",
        avocado.formatted_price(),
        some_number,
        some_string
    );
}

pub(crate) fn main() -> () {
    let avocados = vec![
        Avocado {
            ripeness: Ripeness::Foul,
            price: 0.0,
        },
        Avocado {
            ripeness: Ripeness::Green,
            price: 1.3,
        },
        Avocado {
            ripeness: Ripeness::Perfection,
            price: 2.3,
        },
        Avocado {
            ripeness: Ripeness::Perfection,
            price: 2.0,
        },
    ];



    avocados.iter().for_each(|avocado|{
        match avocado.ripeness {
            Ripeness::HardLikeAStone | Ripeness::SqueezeMe  => {
                println!("This is a stone, not an avocado...");
            }
            Ripeness::MaybeGreen | Ripeness::Green => {
                println!("Keep in on the shelf");
            }
            Ripeness::Perfection => {
                println!("Rip it! And devour it! It's soooo ready! Quickly before it get's Foul. Make it worth the {price}", price=avocado.formatted_price());
            }
            Ripeness::Foul => {
                println!("Toss it, it's not worth: {price}", price=avocado.formatted_price());
            }
        }
    });

    let numbers:Vec<u32> = vec![1,2,6,7];
    let iter_ref = numbers.iter();

    // to create a new vec I just need a reference to the original
    let bigger_numbers: Vec<u32> = iter_ref.map(|num|{
        num + 2
    }).collect();

    let mut numbers = numbers;
    let iter_mut_ref = numbers.iter_mut();

    // mutates numbers in places, which makes it harder to follow the code. sometimes necessary for performance reasons if I can't create a separate copy of numbers
    iter_mut_ref.for_each(|num|{
       *num *= 5
    });

    println!("numbers has changed! {numbers:?}"); // [5, 10, 30, 35]

    let numbers = numbers;
    let owned_iter = numbers.into_iter();

    // need to own value to move it to a new thread
    owned_iter.for_each(|num|{
        std::thread::spawn(move || {
            println!("moved -> {num:?}");
        });
    });






    //








}