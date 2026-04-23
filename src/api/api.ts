import type {
  HeartBeatResponse,
  MessageResponse,
  MusicSearchResponse,
} from "@/types/api";

const baseUrl = process.env.NEXT_PUBLIC_BACKEND_URL;

if (!baseUrl) {
  throw new Error("NEXT_PUBLIC_BACKEND_URL not defined.");
}

export async function getHeartbeat(): Promise<HeartBeatResponse> {
  const res = await fetch(`${baseUrl}/api/heartbeat`, {
    cache: "no-store",
  });

  if (!res.ok) {
    throw new Error("Failed to fetch heartbeat.");
  }

  return res.json();
}

export async function getMusicTest(): Promise<MessageResponse> {
  const res = await fetch(`${baseUrl}/api/music/test`, {
    cache: "no-store",
  });

  if (!res.ok) {
    throw new Error("Failed to fetch music test.");
  }

  return res.json();
}

export async function searchMusic(value: string): Promise<MusicSearchResponse> {
  const res = await fetch(
    `${baseUrl}/api/music/search?value=${encodeURIComponent(value)}`,
    {
      cache: "no-store",
    }
  );

  if (!res.ok) {
    throw new Error("Failed to search music.");
  }

  return res.json();
}