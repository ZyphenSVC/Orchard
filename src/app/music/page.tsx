import { getMusicTest } from "@/api/api"

export default async function MusicPage() {
    const music = await getMusicTest();

    return (
        <main className="p-8">
            <h1 className="text-2xl font-bold">Orchard Music</h1>
            <p className="mt-4">Music route: {music.message}</p>
        </main>
    );
}