use std::io;
use rusqlite::Connection;
use pokedex::models::{Type, Pokemon};
use pokedex::db::{self, get_pokemon_by_type};

fn main() {
    let path = "data/pokedex.db";

    let conn = db::init_db(path).expect("Should have been able to initialize database");

    println!("Welcome to Pokedex");

    loop {
        let mut op_buff = String::new();
        println!("Enter an option. 1: Add. 2: List All, 3: Search by Type, 4. Edit pokemon");
        io::stdin().read_line(&mut op_buff).expect("Should have been able to read from stdio");
        if let Ok(op) = op_buff.trim().parse::<u32>() {
            match op {
                1 => handle_add(&conn),
                2 => {
                    if let Err(e) = handle_list(&conn) {
                        eprintln!("Error listing pokemon: {}", e);
                    }
                },
                3 => handle_search_by_type(&conn).unwrap_or_else(|e| eprintln!("Error listing pokemon: {}", e)),
                4 => handle_edit_pokemon(&conn),
                _ => println!("Invalid option...")
            }
        } else {
            println!("Enter a valid number")
        }
    }
}

fn read_type_from_user() -> Type {
    loop {
        let mut type_buf = String::new();

        println!("Enter the pokemon's type:");
        io::stdin().read_line(&mut type_buf).expect("Should have been able to read from stdio");
        match type_buf.parse::<Type>() {
            Ok(t) => return t,
            Err(e) => {
                println!("Invalid type: {}. Enter a valid Pokemon type", e);
                println!("Valid types include: Psychic, Water, Grass, Fire, Fairy, Normal, Bug, Ghost, Dragon, Electric, Ground, Rock, Dark, Unknown.");
            }
        }
    };
}

fn handle_add(conn: &Connection) {
    let mut name_buf = String::new();

    println!("Enter the Pokemon to add:");
    io::stdin().read_line(&mut name_buf).expect("Should have been able to read from stdio");
    let name_buf = name_buf.trim();


    let has_caught = loop {
        let mut has_caught_buf = String::new();
        println!("Did you catch this Pokemon? 1 (Yes) 2 (No)");
        io::stdin().read_line(&mut has_caught_buf).expect("Should have been able to read from stdio");

        if let Ok(op) = has_caught_buf.trim().parse::<u32>() {
            match op {
                1 => break true,
                2 => break false,
                _ => println!("Enter 1 for Yes and 2 for No"),
            }
        } else {
            println!("Invalid input. Enter 1 for Yes and 2 for No")
        }
    };

    let type_ = read_type_from_user();

    let pokemon = Pokemon {
        name: name_buf.to_string(),
        has_caught,
        type_
    };

    // need to add error handling
    db::add_pokemon(conn, pokemon).expect("Should have been able to add pokemon to database");
}

fn handle_list(conn: &Connection) -> Result<(), rusqlite::Error> {
    let all_pokemon = db::get_all_pokemon(conn).expect("Should have been able to retrieve all pokemon from database");

    for pokemon in all_pokemon {
        println!("{:?}", pokemon);
    }

    Ok(())
}

fn handle_search_by_type(conn: &Connection) -> Result<(), rusqlite::Error> {
    let search_type = read_type_from_user();
    let results = get_pokemon_by_type(conn, search_type)?;

    for pokemon in results {
        println!("{:?}", pokemon);
    }

    Ok(())
}
//TODO: Should probably check to see if the pokemon record exists once the user
//enters the pokemon's name
fn handle_edit_pokemon(conn: &Connection) {
    if let Err(e) = handle_list(&conn) {
        eprintln!("Error listing pokemon: {}", e);
    }
    println!("Enter the name of the pokemon you want to edit:");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Should have been able to read from stdin");
    let buf = buf.trim();

    loop {
        let mut buf2 = String::new();
        println!("Enter the field you would like to edit. 1. Name, 2. has_caught, 3. type");
        io::stdin().read_line(&mut buf2).expect("Should have been able to read from stdin");
        match buf2.trim().parse::<u32>() {
            Ok(1) => { 
                match handle_edit_pokemon_name(&conn, &buf) {
                    Ok(_) => break,
                    Err(e) => eprintln!("Error editing pokemon name: {}", e),
                }
                // I'm fine with breaking out here on the error condition. I think in the future
                // this should be changed to handle user input errors and database errors
                // differently
                break;
            },
            Ok(2) => { 
                match handle_edit_pokemon_has_caught(&conn, &buf) {
                    Ok(_) => break,
                    Err(e) => eprintln!("Error editing pokemon caught status: {}", e),
                }
                break 
            },
            Ok(3) => {
                match handle_edit_pokemon_type(&conn, &buf) {
                    Ok(_) => break,
                    Err(e) => eprintln!("Error editing pokemon Type: {}", e),
                }
                break
            },
            Ok(_) => println!("Invalid option."),
            Err(_) => println!("Enter a valid number")
        }
    }
}
fn handle_edit_pokemon_name(conn: &Connection, pokemon_name: &str) -> Result<(), rusqlite::Error> {
    println!("Enter the new pokemon name");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Should have been able to read from stdin");
    let buf = buf.trim();
    let result = db::edit_pokemon_name(conn, pokemon_name, buf)?;
    match result {
        Some(p) => println!("New value: {:?}", p),
        None => println!("Could not find pokemon"),
    }
    Ok(())
}
fn handle_edit_pokemon_has_caught(conn: &Connection, pokemon_name: &str) -> Result<(), rusqlite::Error> {
    let result = db::edit_pokemon_has_caught(conn, pokemon_name)?;
    match result {
        Some(p) => println!("New value: {:?}", p),
        None => println!("Could not find pokemon"),
    }
    Ok(())
}
fn handle_edit_pokemon_type(conn: &Connection, pokemon_name: &str) -> Result<(), rusqlite::Error> {
    let type_ = read_type_from_user();

    let result = db::edit_pokemon_type(conn, pokemon_name, type_)?;
    match result {
        Some(p) => println!("New value: {:?}", p),
        None => println!("Could not find pokemon"),
    }
    Ok(())
}
