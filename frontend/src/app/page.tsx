// src/app/page.tsx
"use client";
import { useEffect, useState, useCallback, useRef } from "react";
import Image from "next/image";
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
  const [searchQuery, setSearchQuery] = useState("");
  const timeoutRef = useRef<number>();

  const handleError = useCallback((error: unknown) => {
    setLoading(false);
    setError(
      error instanceof Error ? error.message : "An unexpected error occurred",
    );
  }, []);

  const loadInitialData = useCallback(async () => {
    try {
      setLoading(true);
      const data = await rustBridge.invoke<MediaItem[]>("getCatalog");
      setCatalog(data);
      setError(null);
    } catch (error) {
      handleError(error);
    } finally {
      setLoading(false);
    }
  }, [handleError]);

  const handleSearch = useCallback(
    async (query: string) => {
      if (!query.trim()) {
        setSearchResults([]);
        return;
      }

      try {
        setLoading(true);
        const results = await rustBridge.invoke<MediaItem[]>(
          "search",
          query.trim(),
        );
        setSearchResults(results);
        setError(null);
      } catch (error) {
        handleError(error);
        setSearchResults([]);
      } finally {
        setLoading(false);
      }
    },
    [handleError],
  );

  // Safe debounce implementation
  const debouncedSearch = useCallback(
    (query: string) => {
      window.clearTimeout(timeoutRef.current);
      timeoutRef.current = window.setTimeout(() => {
        void handleSearch(query);
      }, 300);
    },
    [handleSearch],
  );

  useEffect(() => {
    void loadInitialData();
    return () => window.clearTimeout(timeoutRef.current);
  }, [loadInitialData]);

  return (
    <div className="mx-auto max-w-4xl p-4">
      <div className="mb-6">
        <input
          type="text"
          placeholder="Search movies..."
          className="w-full rounded border p-2"
          value={searchQuery}
          onChange={(e) => {
            setSearchQuery(e.target.value);
            debouncedSearch(e.target.value);
          }}
          disabled={loading}
          aria-label="Search movies"
        />
      </div>

      {error && (
        <div className="mb-4 rounded bg-red-100 p-4 text-red-700">
          Error: {error}
        </div>
      )}

      {loading ? (
        <div className="p-4 text-gray-500">Loading...</div>
      ) : (
        <MediaGrid items={searchResults.length > 0 ? searchResults : catalog} />
      )}
    </div>
  );
}

function MediaGrid({ items }: { items: MediaItem[] }) {
  return (
    <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
      {items.map((item) => (
        <article
          key={item.id}
          className="overflow-hidden rounded-lg border shadow-sm transition-shadow hover:shadow-md"
        >
          <Image
            src={item.poster}
            alt={item.title}
            width={400}
            height={600}
            className="h-48 w-full object-cover"
            loading="lazy"
          />
          <div className="p-4">
            <h3 className="text-lg font-semibold">{item.title}</h3>
            <p className="text-sm text-gray-600">{item.year}</p>
          </div>
        </article>
      ))}
    </div>
  );
}
