"use client";
import { searchMusic } from "@/api/api"
import { useState } from "react";
export default function SearchClient() {
    const [value, setValue] = useState("");
    const [result, setResult] = useState<string | null>(null);
    const [loading, setLoading] = useState(false);

    async function handleSubmit() {
        if (!value) return;
        setLoading(true);
        try {
            const res = await searchMusic(value);
            setResult(res.value);
        } catch (error) {
            console.error(error);
            setResult("Error fetching result");
        } finally {
            setLoading(false);
        }
    }
    return (
        <div className="mt-6">
            <input
                type="text"
                value={value}
                onChange={(e) => setValue(e.target.value)}
                placeholder="Search for a song..."
                className="border border-gray-300 rounded-md shadow-sm px-2 py-2 leading-tight"
            />
            <button
                onClick={handleSubmit}
                className="bg-black text-white px-2 py-2 leading-tight"
            >
                Search
            </button>
            {loading && <p className="mt-4">Loading...</p>}
            {result && (
                <p className="mt-4">
                    Result: <strong>{result}</strong>
                </p>
            )}
        </div>
    )
}