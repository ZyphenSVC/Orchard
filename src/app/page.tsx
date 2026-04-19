import Image from "next/image";

async function getHealth() {
  const res = await fetch("http://localhost:3001/api/heartbeat", {
    cache: "no-store"
  });

  if(!res.ok) throw new Error("Failed to fetch backend heartbeat");
  return res.json()
}

export default async function Home() {
  const data = await getHealth();
  return (
      <main className="p-8">
        <h1 className="text-2xl font-bold">Rust Music App</h1>
        <p className="mt-4">Backend status: {data.status}</p>
      </main>
  )
}
