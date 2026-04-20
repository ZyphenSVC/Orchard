import { searchMusic } from "@/api/api";

export default async function SearchPage() {
    const result = await searchMusic("tefuashfdasuidhas8dh");

    return (
        <main className="p-8">
            <h1 className="text-2xl font-bold">Search</h1>
            <p className="mt-4">Message: {result.message}</p>
            <p className="mt-2">Value: {result.value}</p>
        </main>
    );
}