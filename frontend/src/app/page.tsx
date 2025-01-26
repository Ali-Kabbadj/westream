"use client";
import { useEffect, useState } from "react";
import { rustBridge } from "@/lib/rust";

type MediaItem = {
  id: string;
  title: string;
  year: number;
  poster: string;
};

export default function Home() {
  const [catalog, setCatalog] = useState<MediaItem[]>([]);
  const [searchResults, setSearchResults] = useState<MediaItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadInitialData = async () => {
      try {
        const data = await rustBridge.invoke<MediaItem[]>("getCatalog");
        setCatalog(data);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Failed to load catalog");
      } finally {
        setLoading(false);
      }
    };

    void loadInitialData();
  }, []);

  const handleSearch = async (query: string) => {
    if (!query) {
      setSearchResults([]);
      return;
    }

    try {
      setLoading(true);
      const results = await rustBridge.invoke<MediaItem[]>("search", { query });
      setSearchResults(results);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Search failed");
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return <div className="p-4">Loading...</div>;
  }

  if (error) {
    return <div className="p-4 text-red-500">Error: {error}</div>;
  }

  return (
    <div className="mx-auto max-w-4xl p-4">
      <div className="mb-6">
        <input
          type="text"
          placeholder="Search movies..."
          className="w-full rounded border p-2"
          onChange={(e) => void handleSearch(e.target.value)}
          disabled={loading}
        />
      </div>

      <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
        {(searchResults.length > 0 ? searchResults : catalog).map((item) => (
          <div
            key={item.id}
            className="overflow-hidden rounded-lg border shadow-sm"
          >
            <img
              src={item.poster}
              alt={item.title}
              className="h-48 w-full object-cover"
              loading="lazy"
            />
            <div className="p-4">
              <h3 className="text-lg font-semibold">{item.title}</h3>
              <p className="text-sm text-gray-600">{item.year}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
