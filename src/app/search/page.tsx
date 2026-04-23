import SearchBox from "./SearchBox";
import { searchMusic } from "@/api/api";

type SearchPageProps = {
  searchParams: Promise<{
    value?: string;
  }>;
};

export default async function SearchPage({ searchParams }: SearchPageProps) {
  const { value } = await searchParams;
  const normalizedValue = value?.trim() ?? "";
  const result = normalizedValue ? await searchMusic(normalizedValue) : null;

  return (
    <main className="p-8">
      <h1 className="text-2xl font-bold">Search</h1>
      <SearchBox />

      {result && (
        <div className="mt-6 space-y-1">
          <p>
            Message: <strong>{result.message}</strong>
          </p>
          <p>
            Value: <strong>{result.value}</strong>
          </p>
          <p>
            Source: <strong>{result.source}</strong>
          </p>
        </div>
      )}
    </main>
  );
}