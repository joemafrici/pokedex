import { useState, useEffect } from 'react'
import { getAllPokemon, addPokemon } from './api/pokemon'
import type { Pokemon, NewPokemon } from './types'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

function App() {
  const [count, setCount] = useState(0)
  const [pokemon, setPokemon] = useState<Pokemon[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [newName, setNewName] = useState("");
  const [newType, setNewType] = useState("");
  const [newHasCaught, setNewHasCaught] = useState(false);

  useEffect(() => {
    async function fetchData() {
      try {
        const data = await getAllPokemon();
        setPokemon(data);
      } catch (err: unknown) {
        if (err instanceof Error) {
          setError(err.message || "Unknown error");
        } else {
          setError("Unknown error occurred");
        }
      } finally {
        setLoading(false);
      }
    }

    fetchData();
  }, []);

  const handleAdd = async (e: React.FormEvent) => {
    e.preventDefault();
    const newPokemon: NewPokemon = {
      name: newName,
      type_: newType,
      has_caught: newHasCaught,
    };

    try {
      const added = await addPokemon(newPokemon);
      setPokemon((prev) => [...prev, added]);
      setNewName("");
      setNewType("");
      setNewHasCaught(false);
    } catch (err: unknown) {
        if (err instanceof Error) {
          setError(err.message);
        } else {
          setError("Unknown error occurred");
        }
    }
  };

  if (loading) return <div>Loading Pokemon...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Pokedex</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>

      <h2>Add a new Pokemon</h2>
      <form onSubmit={handleAdd}>
        <input
          type='text'
          placeholder='Name'
          value={newName}
          onChange={(e) => setNewName(e.target.value)}
          required
        />
        <input
          type='text'
          placeholder='Type'
          value={newType}
          onChange={(e) => setNewType(e.target.value)}
          required
        />
        <label>
          <input
            type='checkbox'
            checked={newHasCaught}
            onChange={(e) => setNewHasCaught(e.target.checked)}
          />
        </label>
        <button type='submit'>Add Pokemon</button>
      </form>
      <ul>
        {pokemon.map((p) => (
          <li key={p.id}>
            {p.name} ({p.type_}) - {p.has_caught ? "Caught" : "Not caught"}
          </li>
        ))}
      </ul>
    </>
  )
}

export default App
