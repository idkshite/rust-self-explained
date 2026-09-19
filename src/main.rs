pub mod closures;
pub mod traits;
mod new_type;
mod lifetimes;
mod iterators;
mod errors;
mod enums;
mod smart_pointers;
mod tests;
mod modules;

use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};
use crate::Cake::{CheeseCake, CherryCake};
use crate::modules::make_cake;

#[derive(Default, Debug)]
enum Cake {
    BlackForestCake,
    CheeseCake,
    #[default]
    CherryCake
}

// impl Default for Cake {
//     fn default() -> Self {
//         CherryCake
//     }
// }

#[derive(Default, Debug)]
struct CakeRecipe {
    cake: Cake,
    // in gram
    weight: u64,
    is_great: bool
}

/*
impl Default for CakeRecipe {
    fn default() -> Self {
        CakeRecipe {
            cake: Cake::default(),
            weight: 1000
        }
    }
}*/

async fn hey_wait(seconds: u64) -> () {
    sleep(Duration::from_secs(seconds)).await;
    println!("I waited {} seconds and it was worth it", seconds)
}

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();


    make_cake();
    tests::main();

}


