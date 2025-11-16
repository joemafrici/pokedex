use std::str::FromStr;
use rusqlite::ToSql;
use rusqlite::types::{ValueRef, ToSqlOutput, FromSql, FromSqlResult};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub has_caught: bool,
    pub type_: Type
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPokemon {
    pub name: String,
    pub has_caught: bool,
    pub type_: Type
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePokemonResponse {
    pub deleted: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePokemon {
    pub current_name: Option<String>,
    pub name: Option<String>,
    pub has_caught: Option<bool>,
    pub type_: Option<Type>,
}

// TODO: add Poison
// add combo typing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
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
