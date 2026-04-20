import Link from "next/link";

export default function Navbar() {
    return (
        <nav className="border-b px-8 py-4">
            <div className="flex gap-6">
                <Link href="/" className="font-medium hover:underline">
                    Home
                </Link>
                <Link href="/music" className="font-medium hover:underline">
                    Music
                </Link>
                <Link href="/search" className="font-medium hover:underline">
                    Search
                </Link>
            </div>
        </nav>
    )
}