import SearchClient from "@/app/search/SearchClient";

export default async function SearchPage() {
    return (
        <main className="p-8">
            <h1 className="text-2xl font-bold">Search</h1>
            <SearchClient />
        </main>
    );
}