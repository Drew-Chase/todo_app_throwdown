import {EmptyState} from "@heroui/react";
import {Icon} from "@iconify-icon/react";
import TodoItem from "./TodoItem.tsx";
import {TodoItem as TodoItemType} from "../../providers/TodoProvider.tsx";

interface TodoListProps
{
    todos: TodoItemType[];
    onToggle: (id: number, checked: boolean) => void;
    onDelete: (id: number) => void;
}

// Ordered stack of todo cards with a friendly empty state when there is
// nothing left to do.
export default function TodoList({todos, onToggle, onDelete}: TodoListProps)
{
    if (todos.length === 0)
    {
        return (
            <EmptyState className={"flex flex-col items-center gap-1 rounded-material bg-surface p-10 text-center shadow-elevation-1"}>
                <Icon icon="material-symbols:checklist-rounded" className={"text-5xl text-accent-soft-foreground"}/>
                <p className={"pt-2 text-base font-medium text-foreground"}>All clear!</p>
                <p className={"text-sm text-muted"}>Add a task above to get started.</p>
            </EmptyState>
        );
    }

    return (
        <div className={"flex flex-col gap-3"}>
            {todos.map(todo => (
                <TodoItem key={todo.id} todo={todo} onToggle={onToggle} onDelete={onDelete}/>
            ))}
        </div>
    );
}
