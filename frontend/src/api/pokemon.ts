import type { Pokemon, NewPokemon } from "../types";

const API_BASE = "http://localhost:3000";

export async function getAllPokemon(): Promise<Pokemon[]> {
  const response = await fetch(`${API_BASE}/api/pokemon`);
  if (!response.ok) {
    throw new Error("Failed to fetch Pokemon");
  }

  const data: Pokemon[] = await response.json();
  return data;
}

export async function addPokemon(newPokemon: NewPokemon): Promise<Pokemon> {
  const response = await fetch(`${API_BASE}/api/pokemon`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(newPokemon),
  });

  if (!response.ok) {
    throw new Error("Failed to fetch Pokemon");
  }

  const data: Pokemon = await response.json();
  return data;
}

export async function updatePokemon(pokemon: Pokemon): Promise<Pokemon> {
  const response = await fetch(`${API_BASE}/api/pokemon/${pokemon.id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(pokemon),
  });

  if (!response.ok) {
    throw new Error("Failed to fetch Pokemon");
  }

  const data: Pokemon = await response.json();
  return data;
}
