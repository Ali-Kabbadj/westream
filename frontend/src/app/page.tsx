// frontend/src/app/page.tsx
"use client";
import { useEffect, useState } from "react";

type MediaItem = {
  id: string;
  title: string;
  year: number;
  poster: string;
};

export default function Home() {
  const [catalog, setCatalog] = useState<MediaItem[]>([]);
  const [searchResults, setSearchResults] = useState<MediaItem[]>([]);

  useEffect(() => {
    // Load initial catalog
    window.__RUST_BRIDGE
      .invoke("getCatalog")
      .then((data) => setCatalog(data))
      .catch(console.error);
  }, []);

  const handleSearch = async (query: string) => {
    const results = await window.__RUST_BRIDGE.invoke("search", query);
    setSearchResults(results);
  };

  return (
    <div className="mx-auto max-w-4xl p-4">
      <input
        type="text"
        placeholder="Search..."
        className="mb-4 w-full rounded border p-2"
        onChange={(e) => handleSearch(e.target.value)}
      />

      <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
        {(searchResults.length > 0 ? searchResults : catalog).map((item) => (
          <div key={item.id} className="overflow-hidden rounded-lg border">
            <img
              src={item.poster}
              alt={item.title}
              className="h-48 w-full object-cover"
            />
            <div className="p-2">
              <h3 className="font-bold">{item.title}</h3>
              <p className="text-sm text-gray-600">{item.year}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
