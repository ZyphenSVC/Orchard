const baseUrl = process.env.NEXT_PUBLIC_BACKEND_URL;

if (!baseUrl) throw new Error("NEXT_PUBLIC_BACKEND_URL not defined.");

export async function getHeartbeat() {
    const res = await fetch(`${baseUrl}/api/heartbeat`, {
        cache: "no-store"
    });

    if(!res.ok) throw new Error("Failed to fetch backend heartbeat");
    return res.json()
}

export async function getMusicTest(): Promise<MessageResponse> {
    const res = await fetch(`${baseUrl}/api/music/test`, {
        cache: "no-store",
    });

    if (!res.ok) {
        throw new Error("Failed to fetch music test");
    }
    return res.json()
}