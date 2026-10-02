import {createContext, ReactNode, useCallback, useContext, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

interface TodoContextType
{
    list: TodoItem[];
    markDone: (id: number) => void;
    markUndone: (id: number) => void;
    insertItem: (title: string, description: string) => Promise<TodoItem>;
    deleteItem: (id: number) => void;

}

export type TodoItem = {
    id: number,
    title: string,
    description: string,
    done: boolean,
}

const TodoContext = createContext<TodoContextType | undefined>(undefined);

export function TodoProvider({children}: { children: ReactNode })
{
    const [items, setItems] = useState<TodoItem[]>([]);

    useEffect(() =>
    {
        invoke("initialize").then(async () => await refreshList());
    }, []);

    const refreshList = async () =>
    {

        const items = await invoke("get_list") as TodoItem[];
        setItems(items);
    };

    const insertItem = useCallback(async (title: string, description: string) =>
    {
        const item = await invoke("insert_item", {title, description}) as TodoItem;
        setItems(prev => [...prev, item]);
        return item;
    }, [items, setItems]);

    const markDone = useCallback(async (id: number) =>
    {
        await invoke("mark_item_as_done", {id});
        setItems(prev => prev.map(i => i.id === id ? {...i, done: true} : i));
    }, [items, setItems]);

    const markUndone = useCallback(async (id: number) =>
    {
        await invoke("mark_item_as_undone", {id});
        setItems(prev => prev.map(i => i.id === id ? {...i, done: false} : i));
    }, [items, setItems]);

    const deleteItem = useCallback(async (id: number) =>
    {
        await invoke("delete_item", {id});
        setItems(prev => prev.filter(i => i.id !== id));
    }, [items, setItems]);

    return (
        <TodoContext.Provider value={{list: items, insertItem, markDone, markUndone, deleteItem}}>
            {children}
        </TodoContext.Provider>
    );
}

export function useTodo(): TodoContextType
{
    const context = useContext(TodoContext);
    if (!context)
    {
        throw new Error("useTodo must be used within a TodoProvider");
    }
    return context;
}