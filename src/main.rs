pub mod closures;
pub mod traits;
mod new_type;
mod lifetimes;
mod iterators;
mod errors;
mod enums;

use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};
use crate::Cake::{CheeseCake, CherryCake};

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

    // let hello = |a: String| { format!("hello {}", a) };
    // let hello = |a| return a;
    //
    // let standard_thingy: CakeRecipe = None.unwrap_or_default();
    //
    // let number_of_yaks = 15;
    // info!(number_of_yaks, "preparing to shave yaks");
    //
    //
    // warn!("hell this may explode");
    // error!(standard_thingy, "good lord this exploded");

    // println!("{:?}", standard_thingy);
    //
    //     hey_wait(5).await;
    // println!("hello {}", hello("world".to_string()));
    // closures::closures_main()
    // closures::iterators_main()
    // traits::traits_main();
    // new_type::main();
    // lifetimes::main();
    // iterators::main();
    // enums::main();

    errors::main();

    /*let res = tokio::spawn(async move {
        // TODO-QUESTION: How to return information from a spawned thread/task?
        // TODO-QUESTION: What's the difference between a thread and a tokio task?
        // errors::main();
        22
    }).await;

    let num = match res {
        Ok(num) => {
            println!("num is {num:?}");
            num
        }
        Err(e) => {
            // TODO: What to do if this is an error?
            eprintln!("this crashed {e:?}");
            // returning the default break intensity
            50
        }
    };

    tokio::spawn(async {
        loop {
            info!("heartbeat");
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });

    tokio::signal::ctrl_c().await.unwrap();*/

    /*tokio::spawn(async {
        loop {
            info!("heartbeat");
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });

    match tokio::spawn(async move {
        errors::main();
    }).await {
        Ok(_) => {
            println!("nothing to report!");
        }
        Err(e) => {
            eprintln!("this crashed {e:?}")
        }
    }

    tokio::signal::ctrl_c().await.unwrap();*/


}
