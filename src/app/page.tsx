import {getHeartbeat, getMusicTest} from "@/api/api";
export default async function Home() {
      const [health, music] = await Promise.all([getHeartbeat(), getMusicTest()])
      return (
          <main className="p-8">
            <h1 className="text-2xl font-bold">Orchard Music</h1>
                <p className="mt-4">Backend status: {health.status}</p>
          </main>
      )
}
