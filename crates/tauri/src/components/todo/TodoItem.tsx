import type {Todo} from "../../types/todo";
import {Button, cn} from "@heroui/react";
import {Icon} from "@iconify-icon/react";
import TodoCheckbox from "./TodoCheckbox.tsx";

interface TodoItemProps
{
    todo: Todo;
    onToggle: (id: string, checked: boolean) => void;
    onDelete: (id: string) => void;
}

// A single todo row: round checkbox, title + optional description and a
// hover-revealed delete action. Completed items are struck through and
// dimmed, Google Tasks style.
export default function TodoItem({todo, onToggle, onDelete}: TodoItemProps)
{
    return (
        <div
            className={cn(
                "group flex items-start gap-3 rounded-material p-4",
                "bg-surface shadow-elevation-1 transition-all duration-300 ease-material",
                "hover:-translate-y-0.5 hover:shadow-elevation-2 motion-reduce:transition-none motion-reduce:hover:translate-y-0",
                todo.completed && "bg-surface-secondary"
            )}
        >
            <TodoCheckbox
                checked={todo.completed}
                onToggle={checked => onToggle(todo.id, checked)}
                label={todo.completed ? `Mark "${todo.title}" as not done` : `Mark "${todo.title}" as done`}
            />
            <div className="flex min-w-0 flex-1 flex-col gap-0.5">
                <p
                    className={cn(
                        "text-base leading-6 font-medium text-foreground transition-colors duration-300 ease-material",
                        todo.completed && "text-muted line-through decoration-accent/60 decoration-2"
                    )}
                >
                    {todo.title}
                </p>
                {todo.description && (
                    <p
                        className={cn(
                            "text-sm leading-5 text-muted transition-colors duration-300 ease-material",
                            todo.completed && "opacity-70"
                        )}
                    >
                        {todo.description}
                    </p>
                )}
            </div>
            <Button
                variant={"ghost"}
                isIconOnly
                size={"sm"}
                aria-label={`Delete "${todo.title}"`}
                className={cn(
                    "mt-0.5 size-8 shrink-0 rounded-full text-muted opacity-0 transition-all duration-200 ease-material",
                    "group-hover:opacity-100 focus-visible:opacity-100 hover:bg-danger-soft hover:text-danger",
                    "motion-reduce:transition-none"
                )}
                onPress={() => onDelete(todo.id)}
            >
                <Icon icon="material-symbols:delete-outline-rounded" className={"text-lg"}/>
            </Button>
        </div>
    );
}
