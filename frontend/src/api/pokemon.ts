import type { Pokemon } from "../types";

const API_BASE = "http://localhost:3000";

export async function getAllPokemon(): Promise<Pokemon[]> {
  const response = await fetch(`${API_BASE}/api/pokemon`);
  if (!response.ok) {
    throw new Error("Failed to fetch Pokemon");
  }

  const data: Pokemon[] = await response.json();
  return data;
}
