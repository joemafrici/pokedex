import { useState } from "react";
//import type { Pokemon } from "../types";
import { addPokemon } from "../api/pokemon";

//type AddPokemonFormProps = {
//  onAdd: (pokemon: Omit<Pokemon, "id">) => Promise<void>;
//};

export default function AddPokemonForm() {
  const [name, setName] = useState("");
  const [hasCaught, setHasCaught] = useState(false);
  const [type_, setType] = useState("");

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();

    await addPokemon({
      name,
      has_caught: hasCaught,
      type_: type_,
    });

    setName("");
    setHasCaught(false);
    setType("Unknown");
  }

  return (
      <>
        <h2>Add a new Pokemon</h2>
        <form onSubmit={handleSubmit}>
          <input
            type='text'
            placeholder='Name'
            value={name}
            onChange={(e) => setName(e.target.value)}
            required
          />
          <input
            type='text'
            placeholder='Type'
            value={type_}
            onChange={(e) => setType(e.target.value)}
            required
          />
          <label>
            Caught?
            <input
              type='checkbox'
              checked={hasCaught}
              onChange={(e) => setHasCaught(e.target.checked)}
            />
          </label>
          <button type='submit'>Add Pokemon</button>
        </form>
      </>
  );
}
