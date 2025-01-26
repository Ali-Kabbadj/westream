// src/app/page.tsx
"use client";
import { useEffect, useState, useCallback } from "react";
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
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadInitialData = useCallback(async () => {
    try {
      setLoading(true);
      const data = await rustBridge.invoke<MediaItem[]>("getCatalog");
      setCatalog(data);
      setError(null);
    } catch (error) {
      setError(
        error instanceof Error ? error.message : "Failed to load catalog",
      );
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void loadInitialData();
  }, [loadInitialData]);

  return (
    <div className="mx-auto max-w-4xl">
      {error && (
        <div className="mb-4 rounded bg-red-100 p-4 text-red-700">
          Error: {error}
        </div>
      )}

      {loading ? (
        <div className="p-4 text-gray-500">Loading...</div>
      ) : (
        <MediaGrid items={catalog} />
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
            priority
            src={item.poster}
            alt={item.title}
            width={400}
            height={600}
            className="h-48 w-full object-cover"
            // Add placeholder for broken images
            onError={(e) => {
              (e.target as HTMLImageElement).src = "/fallback-image.jpg";
            }}
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
