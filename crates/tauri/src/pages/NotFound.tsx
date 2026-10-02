import {Link} from "react-router-dom";

export default function NotFound()
{
    return (
        <div className="flex-1 flex flex-col items-center justify-center gap-2 p-6">
            <h1 className="text-2xl font-bold">404</h1>
            <p className="text-foreground/60">This page doesn't exist.</p>
            <Link to="/" className="text-accent">Back home</Link>
        </div>
    );
}
