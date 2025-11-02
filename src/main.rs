use std::str::FromStr;
use std::io;
use rusqlite::{Connection, ToSql};
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
        println!("Enter an option. 1: Add. 2: List");
        io::stdin().read_line(&mut op_buff).expect("Should have been able to read from stdio");
        let op: u32 = op_buff.trim().parse().expect("Should have been able to convert user menu option input to integer. Maybe the user entered not a number?");
        match op {
            1 => handle_add(&conn),
            2 => handle_list(&conn),
            _ => continue
        }
    }
}
fn handle_add(conn: &Connection) {
    let mut buf = String::new();
    let mut buf2 = String::new();
    let mut buf3 = String::new();

    println!("Enter the Pokemon to add:");
    io::stdin().read_line(&mut buf).expect("Should have been able to read from stdio");

    println!("Did you catch this Pokemon? 1 (Yes) 2 (No)");
    io::stdin().read_line(&mut buf2).expect("Should have been able to read from stdio");
    let op: u32 = buf2.trim().parse().expect("Should have been able to convert user menu option input to integer. Maybe the user entered not a number?");
    let has_caught;
    match op {
        1 => has_caught = true,
        _ => has_caught = false,
    }

    println!("Enter the pokemon's type:");
    io::stdin().read_line(&mut buf3).expect("Should have been able to read from stdio");

    let pokemon = Pokemon {
        name: buf,
        has_caught,
        type_: buf3.parse().expect("Should have been able to parse user supplied type to Type object"),
    };

    conn.execute("INSERT INTO pokemon (name, has_caught, type) VALUES (?1, ?2, ?3)", (pokemon.name, pokemon.has_caught, pokemon.type_)).expect("Should have been able to add pokemon to database");
}

fn handle_list(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT name, has_caught, type FROM pokemon").expect("Should have been able to query database for all pokemon");
    let pokemon_iter = stmt.query_map([], |row| {
        Ok(Pokemon {
            name: row.get("name").expect("Should have been able to get name"),
            has_caught: row.get("has_caught").expect("Should have been able to get has_caught"),
            type_: row.get("type").expect("Should have been able to get type"),
        })
    }).expect("Should have been able to convert database results to rust objects");
    for pokemon in pokemon_iter {
        println!("Found pokemon {:?}", pokemon.expect("Should have been able to get pokemon"));
    }
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
