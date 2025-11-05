use std::str::FromStr;
use std::io;
use rusqlite::{params, Connection, ToSql};
use rusqlite::types::{ValueRef, FromSql, FromSqlResult, ToSqlOutput};

fn main() {
    let conn = Connection::open_in_memory().expect("Should have been able to open database connection");
    conn.execute("CREATE TABLE IF NOT EXISTS pokemon (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            has_caught BOOLEAN NOT NULL,
            type TEXT NOT NULL
        )", (),
    ).expect("Should have been able to create pokemon table");

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

    let type_ = loop {
        let mut type_buf = String::new();

        println!("Enter the pokemon's type:");
        io::stdin().read_line(&mut type_buf).expect("Should have been able to read from stdio");
        match type_buf.parse::<Type>() {
            Ok(t) => break t,
            Err(e) => {
                println!("Invalid type: {}. Enter a valid Pokemon type", e);
                println!("Valid types include: Psychic, Water, Grass, Fire, Fairy, Normal, Bug, Ghost, Dragon, Electric, Ground, Rock, Dark, Unknown.");
            }
        }
    };

    let pokemon = Pokemon {
        name: name_buf.to_string(),
        has_caught,
        type_
    };

    match conn.execute("INSERT INTO pokemon (name, has_caught, type) VALUES (?1, ?2, ?3)", (pokemon.name, pokemon.has_caught, pokemon.type_)) {
        Ok(_) => println!("Pokemon added successfully"),
        Err(e) => eprintln!("Failed to add Pokemon to database: {}", e),
    }
}

fn handle_list(conn: &Connection) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT name, has_caught, type FROM pokemon")?;
    let pokemon_iter = stmt.query_map([], |row| {
        Ok(Pokemon {
            name: row.get("name")?,
            has_caught: row.get("has_caught")?,
            type_: row.get("type")?,
        })
    })?;
    for pokemon_result in pokemon_iter {
        match pokemon_result {
            Ok(pokemon) => println!("Found pokemon: {:?}", pokemon),
            Err(e) => eprintln!("Failed to read Pokemon row: {}", e)
        }
    }
    Ok(())
}

fn handle_search_by_type(conn: &Connection) -> Result<(), rusqlite::Error> {
    println!("Enter the pokemon type to search for:");

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Should have been able to read from stdin");
    let search_type: Type = buf.parse().expect("Should have been able to parse search type");
    let mut stmt = conn.prepare("SELECT name, has_caught, type FROM pokemon WHERE type = ?1")?;

    let pokemon_iter = stmt.query_map(params![search_type], |row| {
        Ok(Pokemon {
            name: row.get("name")?,
            has_caught: row.get("has_caught")?,
            type_: row.get("type")?,
        })
    })?;

    for pokemon_result in pokemon_iter {
        match pokemon_result {
            Ok(pokemon) => println!("Found pokemon {:?}", pokemon),
            Err(e) => eprintln!("Failed to read pokemon row: {}", e),
        }
    }

    Ok(())
}
fn handle_edit_pokemon(conn: &Connection) {
    if let Err(e) = handle_list(&conn) {
        eprintln!("Error listing pokemon: {}", e);
    }
    println!("Enter the name of the pokemon you want to edit:");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Should have been able to read from stdin");
    let buf = buf.trim();

    println!("Enter the field you would like to edit. 1. Name, 2. has_caught, 3. type");
    let mut buf2 = String::new();
    io::stdin().read_line(&mut buf2).expect("Should have been able to read from stdin");
    let op: u32 = buf2.trim().parse().expect("Should have been able to convert user menu option input to integer. Maybe the user entered not a number?");
    match op {
        1 => handle_edit_pokemon_name(&conn, &buf),
        2 => handle_edit_pokemon_has_caught(&conn, &buf),
        3 => handle_edit_pokemon_type(&conn, &buf),
        _ => println!("Invalid input."),
    }
}
// need to change this to return a result eventually
fn handle_edit_pokemon_name(conn: &Connection, pokemon_name: &str) {
    println!("Enter the new pokemon name");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Should have been able to read from stdin");

    let rows_updated= conn.execute("UPDATE pokemon SET name = ?2 WHERE name = ?1", params![pokemon_name, buf.trim()]).expect("Should have been able to prepare sql query");
    println!("Updated {} row(s)", rows_updated);
}
fn handle_edit_pokemon_has_caught(conn: &Connection, pokemon_name: &str) {
    let rows_updated = conn.execute("UPDATE pokemon SET has_caught = NOT has_caught WHERE name = ?1", params![pokemon_name]).expect("Should have been able to toggle has_caught value");
    if rows_updated == 1 {
        println!("has_caught value toggled");
    } else {
        println!("Did not find record to update");
    }
}
fn handle_edit_pokemon_type(conn: &Connection, pokemon_name: &str) {
    println!("Enter the new pokemon type");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Should have been able to read from stdin");
    let type_: Type = buf.parse().expect("Should hav been able to parse Type");

    let rows_updated= conn.execute("UPDATE pokemon SET type = ?2 WHERE name = ?1", params![pokemon_name, type_]).expect("Should have been able to prepare sql query");
    println!("Updated {} row(s)", rows_updated);
}

#[derive(Debug, Clone)]
struct Pokemon {
    name: String,
    has_caught: bool,
    type_: Type
}

#[derive(Debug, Clone)]
enum Type {
    Psychic,
    Water,
    Grass,
    Fire,
    Fairy,
    Normal,
    Bug,
    Ghost,
    Dragon,
    Electric,
    Ground,
    Rock,
    Dark,
    Unknown,
}
impl FromStr for Type {
    type Err = String;
    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "psychic" => Ok(Type::Psychic),
            "water" => Ok(Type::Water),
            "grass" => Ok(Type::Grass),
            "fire" => Ok(Type::Fire),
            "fairy" => Ok(Type::Fairy),
            "normal" => Ok(Type::Normal),
            "bug" => Ok(Type::Bug),
            "ghost" => Ok(Type::Ghost),
            "dragon" => Ok(Type::Dragon),
            "electric" => Ok(Type::Electric),
            "ground" => Ok(Type::Ground),
            "rock" => Ok(Type::Rock),
            "dark" => Ok(Type::Dark),
            "unknown" => Ok(Type::Unknown),
            _ => Err(format!("Unknown pokemon type: {}", s)),
        }
    }
}
impl FromSql for Type {
    // Required method
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "Psychic" => Ok(Type::Psychic),
            "Water" => Ok(Type::Water),
            "Grass" => Ok(Type::Grass),
            "Fire" => Ok(Type::Fire),
            "Fairy" => Ok(Type::Fairy),
            "Normal" => Ok(Type::Normal),
            "Bug" => Ok(Type::Bug),
            "Ghost" => Ok(Type::Ghost),
            "Dragon" => Ok(Type::Dragon),
            "Electric" => Ok(Type::Electric),
            "Ground" => Ok(Type::Ground),
            "Rock" => Ok(Type::Rock),
            "Dark" => Ok(Type::Dark),
            _ => Ok(Type::Unknown),
        }
    }
}

impl ToSql for Type {
    // Required method
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let text = match self {
            Type::Psychic => "Psychic",
            Type::Water => "Water",
            Type::Grass => "Grass",
            Type::Fire => "Fire",
            Type::Fairy => "Fairy",
            Type::Normal => "Normal",
            Type::Bug => "Bug",
            Type::Ghost => "Ghost",
            Type::Dragon => "Dragon",
            Type::Electric => "Electric",
            Type::Ground => "Ground",
            Type::Rock => "Rock",
            Type::Dark => "Dark",
            Type::Unknown => "Unknown",
        };
        Ok(ToSqlOutput::from(text))
    }
}
