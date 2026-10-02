// Shared shape of a todo item. The backend (Tauri command / Actix API)
// should serialize to exactly this so the UI layer stays decoupled.
export interface Todo
{
    id: string;
    title: string;
    description?: string;
    completed: boolean;
}
