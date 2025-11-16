use crate::models::{Pokemon, NewPokemon, Type};
use rusqlite::{Connection, Result, params};

pub fn init_db(path: &str) -> Result<rusqlite::Connection> {
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pokemon (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            has_caught BOOLEAN NOT NULL,
            type TEXT NOT NULL
        )",
        (),
    )?;
    Ok(conn)
}
pub fn add_pokemon(conn: &Connection, pokemon: NewPokemon) -> Result<Pokemon> {
    let id: u32 = conn.query_row("INSERT INTO pokemon (name, has_caught, type) VALUES (?1, ?2, ?3) RETURNING id;", (&pokemon.name, &pokemon.has_caught, &pokemon.type_), |row| row.get("id"))?;
    Ok(Pokemon{
        id,
        name: pokemon.name,
        has_caught: pokemon.has_caught,
        type_: pokemon.type_
    })
}
pub fn get_all_pokemon(conn: &Connection) -> Result<Vec<Pokemon>> {
    let mut stmt = conn.prepare("SELECT id, name, has_caught, type FROM pokemon")?;
    let rows = stmt.query_map([], |row| {
        Ok(Pokemon {
            id: row.get("id")?,
            name: row.get("name")?,
            has_caught: row.get("has_caught")?,
            type_: row.get("type")?,
        })
    })?;
    Ok(rows.filter_map(Result::ok).collect())
}
pub fn get_pokemon_by_id(conn: &Connection, id: u32) -> Result<Option<Pokemon>> {
    let mut stmt = conn.prepare("SELECT id, name, has_caught, type FROM pokemon WHERE id = ?1")?;
    let pokemon = stmt.query_row(params![id], |row| {
        Ok(Pokemon {
            id: row.get("id")?,
            name: row.get("name")?,
            has_caught: row.get("has_caught")?,
            type_: row.get("type")?,
        })
    })?;

    Ok(Some(pokemon))
}
pub fn edit_pokemon_by_id(conn: &Connection, id: u32, pokemon: Pokemon) -> Result<Option<Pokemon>> {
    let mut stmt= conn.prepare("UPDATE pokemon SET name = ?1, has_caught = ?2, type = ?3 WHERE id = ?4 RETURNING id, name, has_caught, type")?;


    let result = stmt.query_row(params![pokemon.name, pokemon.has_caught, pokemon.type_, id], |row| {
        Ok(Pokemon {
            id: row.get("id")?,
            name: row.get("name")?,
            has_caught: row.get("has_caught")?,
            type_: row.get("type")?,
        })
    });

    match result {
        Ok(p) => Ok(Some(p)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }

}
pub fn get_pokemon_by_name(conn: &Connection, name: &str) -> Result<Vec<Pokemon>> {
    let mut stmt = conn.prepare("SELECT id, name, has_caught, type FROM pokemon WHERE name = ?1")?;

    let rows = stmt.query_map(params![name], |row| {
        Ok(Pokemon {
            id: row.get("id")?,
            name: row.get("name")?,
            has_caught: row.get("has_caught")?,
            type_: row.get("type")?,
        })
    })?;
    Ok(rows.filter_map(Result::ok).collect())
}
pub fn get_pokemon_by_type(conn: &Connection, type_: Type) -> Result<Vec<Pokemon>> {
    let mut stmt = conn.prepare("SELECT id, name, has_caught, type FROM pokemon WHERE type = ?1")?;

    let rows = stmt.query_map(params![type_], |row| {
        Ok(Pokemon {
            id: row.get("id")?,
            name: row.get("name")?,
            has_caught: row.get("has_caught")?,
            type_: row.get("type")?,
        })
    })?;
    Ok(rows.filter_map(Result::ok).collect())
}
pub fn edit_pokemon_name(conn: &Connection, current: &str, new: &str) -> Result<Option<Pokemon>> {
    let rows_updated = conn.execute("UPDATE pokemon SET name = ?2 WHERE name = ?1", params![current, new])?;

    if rows_updated == 0 {
        return Ok(None);
    }

    let mut stmt = conn.prepare("SELECT id, name, has_caught, type FROM pokemon WHERE name = ?1")?;
    let pokemon = stmt.query_row(params![new], |row| {
        Ok(Pokemon {
            id: row.get("id")?,
            name: row.get("name")?,
            has_caught: row.get("has_caught")?,
            type_: row.get("type")?,
        })
    })?;

    Ok(Some(pokemon))
}
// TODO: This function only returns the first result despite the possibility that there are
// multiple results. I'm leaning towards not allowing dupliate pokemon to exist
pub fn edit_pokemon_has_caught(conn: &Connection, name: &str) -> Result<Option<Pokemon>> {
    let rows_updated = conn.execute("UPDATE pokemon SET has_caught = NOT has_caught WHERE name = ?1", params![name])?;

    if rows_updated == 1 {
        let results = get_pokemon_by_name(conn, name)?;
        Ok(results.first().cloned())
    } else {
        Ok(None)
    }
}
pub fn edit_pokemon_type(conn: &Connection, name: &str, type_: Type) -> Result<Option<Pokemon>> {
    let rows_updated= conn.execute("UPDATE pokemon SET type = ?2 WHERE name = ?1", params![name, type_])?;
    if rows_updated == 1 {
        let results = get_pokemon_by_name(conn, name)?;
        Ok(results.first().cloned())
    } else {
        Ok(None)
    }
}
