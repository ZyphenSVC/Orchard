import {getHeartbeat} from "@/api/api";
export default async function Home() {
      const data = await getHeartbeat();
      return (
          <main className="p-8">
            <h1 className="text-2xl font-bold">Rust Music App</h1>
            <p className="mt-4">Backend status: {data.status}</p>
          </main>
      )
}
