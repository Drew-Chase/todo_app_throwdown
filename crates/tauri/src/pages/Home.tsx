import {TodoComposer, TodoHeader, TodoList} from "../components/todo";
import {useTodo} from "../providers/TodoProvider.tsx";

export default function Home()
{
    const {list, insertItem, markUndone, markDone, deleteItem} = useTodo();

    const remaining = list.filter(todo => !todo.done).length;

    return (
        <div className={"mx-auto flex w-full max-w-xl flex-col gap-6 px-4 py-8"}>
            <TodoHeader total={list.length} remaining={remaining}/>
            <TodoComposer onAdd={insertItem}/>
            <TodoList
                todos={list}
                onToggle={(id, done) =>
                {
                    if (done) markDone(id);
                    else markUndone(id);
                }}
                onDelete={deleteItem}
            />
        </div>
    );
}
