import type {FormEvent} from "react";
import {useState} from "react";
import {Button, Input, TextArea} from "@heroui/react";
import {Icon} from "@iconify-icon/react";

interface TodoComposerProps
{
    onAdd: (title: string, description: string) => void;
}

// Material-style "quick add" card: a borderless title field, an optional
// description area and a round accent action button. Calls `onAdd` with the
// trimmed values and clears itself; pure UI state, no persistence.
export default function TodoComposer({onAdd}: TodoComposerProps)
{
    const [title, setTitle] = useState("");
    const [description, setDescription] = useState("");

    const canSubmit = title.trim().length > 0;

    const handleSubmit = (event: FormEvent<HTMLFormElement>) =>
    {
        event.preventDefault();
        if (!canSubmit)
        {
            return;
        }
        onAdd(title.trim(), description.trim());
        setTitle("");
        setDescription("");
    };

    return (
        <form
            onSubmit={handleSubmit}
            className={
                "flex flex-col gap-1 rounded-material bg-surface p-4 shadow-elevation-2 transition-shadow duration-300 ease-material focus-within:shadow-elevation-3"
            }
        >
            <Input
                aria-label={"Task title"}
                placeholder={"What needs doing?"}
                value={title}
                onChange={e => setTitle(e.target.value)}
                className={
                    "border-0 bg-transparent px-1 text-lg font-medium text-foreground shadow-none " +
                    "placeholder:font-normal placeholder:text-field-placeholder focus:bg-transparent"
                }
            />
            <TextArea
                aria-label={"Task description"}
                placeholder={"Add details (optional)"}
                rows={2}
                value={description}
                onChange={e => setDescription(e.target.value)}
                className={
                    "resize-none border-0 bg-transparent px-1 text-sm text-foreground shadow-none " +
                    "placeholder:text-field-placeholder focus:bg-transparent"
                }
            />
            <div className={"flex items-center justify-between pt-1"}>
                <p className={"pl-1 text-xs text-muted"}>Press Enter to add</p>
                <Button
                    type={"submit"}
                    variant={"primary"}
                    isIconOnly
                    isDisabled={!canSubmit}
                    aria-label={"Add task"}
                    className={
                        "size-10 rounded-full shadow-elevation-1 transition-transform duration-200 ease-material " +
                        "hover:scale-105 active:scale-95 motion-reduce:transition-none motion-reduce:hover:scale-100"
                    }
                >
                    <Icon icon="material-symbols:add-rounded" className={"text-2xl"}/>
                </Button>
            </div>
        </form>
    );
}
