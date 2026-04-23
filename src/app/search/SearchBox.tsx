"use client";

import { useState } from "react";
import { useRouter, useSearchParams } from "next/navigation";

export default function SearchBox() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const initialValue = searchParams.get("value") ?? "";
  const [value, setValue] = useState(initialValue);

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();

    const trimmed = value.trim();

    if (!trimmed) {
      router.push("/search");
      return;
    }

    router.push(`/search?value=${encodeURIComponent(trimmed)}`);
  }

  return (
    <form onSubmit={handleSubmit} className="mt-6 flex gap-2">
      <input
        type="text"
        value={value}
        onChange={(e) => setValue(e.target.value)}
        placeholder="Search for a song..."
        className="border border-gray-300 rounded-md shadow-sm px-3 py-2 leading-tight"
      />
      <button
        type="submit"
        className="bg-black text-white px-4 py-2 rounded-md"
      >
        Search
      </button>
    </form>
  );
}