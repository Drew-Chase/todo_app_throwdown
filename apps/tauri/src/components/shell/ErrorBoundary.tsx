import React from "react";
import {Button} from "@heroui/react";

interface Props
{
    children: React.ReactNode;
}

interface State
{
    error: Error | null;
}

// Per-page error boundary. Catches render-time exceptions in its children
// and shows a recoverable error card instead of crashing the entire window.
// Route navigation resets it automatically because the route's component
// key changes; "Reload page" below clears the error state in place.
export default class ErrorBoundary extends React.Component<Props, State>
{
    constructor(props: Props)
    {
        super(props);
        this.state = {error: null};
    }

    static getDerivedStateFromError(error: Error): State
    {
        return {error};
    }

    componentDidCatch(error: Error, info: React.ErrorInfo)
    {
        console.error("[ErrorBoundary] Caught:", error, info.componentStack);
    }

    render()
    {
        if (this.state.error)
        {
            return (
                <div className="flex-1 flex flex-col items-center justify-center gap-4 p-10 bg-background">
                    <div className="w-full max-w-[480px] rounded-xl border border-danger/20 bg-danger/5 p-7">
                        <div className="text-base font-bold text-danger mb-2">
                            Something went wrong
                        </div>
                        <div className="text-xs text-foreground/70 leading-relaxed mb-4">
                            An unexpected error occurred while rendering this page.
                            The rest of the application is unaffected.
                        </div>
                        <pre className="text-[11px] font-mono text-foreground/60 bg-black/20 rounded-lg p-3 overflow-auto max-h-40 mb-4 whitespace-pre-wrap break-words">
                            {this.state.error.message}
                            {this.state.error.stack && `\n\n${this.state.error.stack}`}
                        </pre>
                        <Button
                            size="sm"
                            variant="tertiary"
                            className="border-separator text-foreground"
                            onPress={() => this.setState({error: null})}
                        >
                            Reload page
                        </Button>
                    </div>
                </div>
            );
        }

        return this.props.children;
    }
}
