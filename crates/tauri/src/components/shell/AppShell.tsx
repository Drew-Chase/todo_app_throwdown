import {Outlet} from "react-router-dom";
import WindowChrome from "./WindowChrome.tsx";
import ErrorBoundary from "./ErrorBoundary.tsx";

// Layout for all in-app routes: titlebar on top, nav rail on the left,
// routed page content on the right, each page isolated by its own
// error boundary so a render crash on one page doesn't take down the shell.
export default function AppShell()
{
    return (
        <div className="flex flex-col h-screen w-screen overflow-hidden bg-background">
            <WindowChrome/>
            <div className="flex flex-row flex-1 min-h-0">
                <div className="flex flex-col flex-1 min-w-0 min-h-0 overflow-y-auto">
                    <ErrorBoundary>
                        <Outlet/>
                    </ErrorBoundary>
                </div>
            </div>
        </div>
    );
}
