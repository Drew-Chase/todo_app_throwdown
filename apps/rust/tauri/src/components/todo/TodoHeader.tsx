import {Chip} from "@heroui/react";

interface TodoHeaderProps
{
    total: number;
    remaining: number;
}

// Page heading with a live progress pill: how many tasks are still open.
export default function TodoHeader({total, remaining}: TodoHeaderProps)
{
    const allDone = total > 0 && remaining === 0;

    return (
        <div className={"flex items-center gap-3"}>
            <h1 className={"text-4xl font-bold tracking-tight text-foreground"}>Tasks</h1>
            {total > 0 && (
                <Chip
                    color={allDone ? "success" : "accent"}
                    variant={"soft"}
                    size={"sm"}
                    className={"rounded-full"}
                >
                    {allDone ? "All done" : `${remaining} of ${total} left`}
                </Chip>
            )}
        </div>
    );
}
