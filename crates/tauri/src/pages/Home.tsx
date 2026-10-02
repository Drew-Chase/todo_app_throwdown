import {useState} from "react";
import type {Todo} from "../types/todo";
import {TodoComposer, TodoHeader, TodoList} from "../components/todo";

// Seed data so the UI renders with content. Once the backend is hooked up,
// replace `todos`/handlers with the Tauri command / API results — every
// child component only communicates through the props below.
const INITIAL_TODOS: Todo[] = [
    {
        id: "demo-1",
        title: "Try checking me off",
        description: "Click the round checkbox to mark this task as done.",
        completed: false
    },
    {
        id: "demo-2",
        title: "Add your own task",
        description: "Type a title, optionally add a description, then hit Enter or the + button.",
        completed: false
    },
    {
        id: "demo-3",
        title: "Hover a task to delete it",
        completed: false
    },
    {
        id: "demo-4",
        title: "This one is already done",
        description: "Completed tasks are struck through and dimmed.",
        completed: true
    }
];

export default function Home()
{
    const [todos, setTodos] = useState<Todo[]>(INITIAL_TODOS);

    const handleAdd = (title: string, description: string) =>
    {
        setTodos(previous => [
            {
                id: crypto.randomUUID(),
                title,
                description: description.length > 0 ? description : undefined,
                completed: false
            },
            ...previous
        ]);
    };

    const handleToggle = (id: string, checked: boolean) =>
    {
        setTodos(previous => previous.map(todo => (todo.id === id ? {...todo, completed: checked} : todo)));
    };

    const handleDelete = (id: string) =>
    {
        setTodos(previous => previous.filter(todo => todo.id !== id));
    };

    const remaining = todos.filter(todo => !todo.completed).length;

    return (
        <div className={"mx-auto flex w-full max-w-xl flex-col gap-6 px-4 py-8"}>
            <TodoHeader total={todos.length} remaining={remaining}/>
            <TodoComposer onAdd={handleAdd}/>
            <TodoList todos={todos} onToggle={handleToggle} onDelete={handleDelete}/>
        </div>
    );
}
