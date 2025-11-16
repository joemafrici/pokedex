import { useState, useEffect } from 'react'
import { getAllPokemon, addPokemon, updatePokemon } from './api/pokemon'
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

  const [editingId, setEditingId] = useState<number | null>(null);
  const [editName, setEditName] = useState("");
  const [editType, setEditType] = useState("");
  const [editHasCaught, setEditHasCaught] = useState(false);

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

  const startEditing = (p: Pokemon) => {
    setEditingId(p.id);
    setEditName(p.name);
    setEditType(p.type_);
    setEditHasCaught(p.has_caught);
  };
  const handleUpdate = async (e: React.FormEvent) => {
    e.preventDefault();

    if (editingId === null) return;

    const updated: Pokemon = {
      id: editingId,
      name: editName,
      type_: editType,
      has_caught: editHasCaught,
    };

    try {
      const result = await updatePokemon(updated);

      setPokemon((prev) => prev.map((p) => (p.id === result.id ? result : p)));
      setEditingId(null);
    } catch (err: unknown) {
      if (err instanceof Error) setError(err.message);
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
          Caught?
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
            {editingId === p.id? (
              <form onSubmit={handleUpdate}>
                <input
                  value={editName}
                  onChange={(e) => setEditName(e.target.value)}
                />
                <input
                  value={editType}
                  onChange={(e) => setEditType(e.target.value)}
                />
                <label>
                  Caught?
                  <input
                    type='checkbox'
                    checked={editHasCaught}
                    onChange={(e) => setEditHasCaught(e.target.checked)}
                  />
                </label>
                <button type='submit'>Save</button>
                <button type='button' onClick={() => setEditingId(null)}>
                  Cancel
                </button>
              </form>
            ) : (
              <>
                {p.name} ({p.type_}) - {p.has_caught ? "Caught" : "Not caught"}
                <button onClick={() => startEditing(p)}>Edit</button>
              </>
            )}
          </li>
        ))}
      </ul>
    </>
  )
}

export default App
