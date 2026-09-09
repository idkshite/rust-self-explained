use std::fs::{File, read_dir, read};
use std::path::Path;
use std::env::current_dir;

// TODO-QUESTION: What to do if I need to return multiple error types from one function?

enum Meals {
    SpaghettiBolognese,
    Oatmeal,
    BreadnButter
}

#[derive(Debug, Copy, Clone)]
enum ProcedureError{
    PathObstructed,
    LowPower,
    ObjectNotFound,
    RoomNotFound,
}

pub(crate) fn main(){
    // panic!("ayayaya")
    // safe_read_file();

    send_robot_order();
}

fn send_robot_order(){
    // go to the kitchen
    move_to("Kitttttchen".to_string()).unwrap_or_else(|error| {
        match error {
            ProcedureError::RoomNotFound => {
                move_to("Base".to_string()).expect("Base should always be avaialable");
                panic!("RoomNotFound: Failed to move to kitchen. Went back to base")
            }
            others => {
                panic!("Failed to move to kitchen. {error:?}")
            }
        }
    });
    // cook meal XY67AC
    cook_meal(Meals::Oatmeal).unwrap();
    // serve the meal
    serve_meal(Meals::Oatmeal).unwrap();
}

fn move_to(room: String) -> Result<(), ProcedureError>{
    if(room == "Base".to_string()){
        println!("Moving to Base");
        return Ok(())
    }
    if(room != "Kitchen".to_string()){
       return Err(ProcedureError::RoomNotFound)
    }

    Ok(())
}

fn cook_meal(meal: Meals) -> Result<i32, ProcedureError>{
    Ok(23)
}

fn serve_meal(meal: Meals) -> Result<(), ProcedureError>{
    Ok(())
}

fn safe_read_file(){
    println!("{:?}", current_dir().unwrap());
    let og_path = "./../files/dogs.txt";
    let new_file = read(og_path);

    new_file.expect("dogs text file should be part of the repo");

    /*let content = new_file.unwrap_or_else(|err|{

        eprintln!("Read File Error: {err:?}");
        let path = Path::new(og_path).parent().expect("to parse a valid path");
        let paths = read_dir(path).unwrap_or_else(|err|{
            panic!("Paths Error: {err:?}")

        });

        for single_path in paths {
            println!("Did you mean? {}", single_path.unwrap().path().display())
        }

        panic!("Invalid Path: {err}")
        //vec![]

    });

    println!("{:?}", str::from_utf8(&content).unwrap()) */
}