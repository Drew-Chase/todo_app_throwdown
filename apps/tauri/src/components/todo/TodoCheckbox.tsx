import {Checkbox} from "@heroui/react";

interface TodoCheckboxProps
{
    checked: boolean;
    onToggle: (checked: boolean) => void;
    label: string;
}

// Round "Google Tasks" style checkbox. The HeroUI checkbox fills with the
// accent color and draws an animated checkmark when selected; here we only
// reshape the square control into a circle and enlarge it one step.
export default function TodoCheckbox({checked, onToggle, label}: TodoCheckboxProps)
{
    return (
        <Checkbox
            isSelected={checked}
            onChange={onToggle}
            aria-label={label}
            className={"w-fit"}
        >
            <Checkbox.Content className={"rounded-full p-0.5 [&_[data-slot=checkbox-default-indicator--checkmark]]:size-3"}>
                <Checkbox.Control className={"size-5 rounded-full"}>
                    <Checkbox.Indicator/>
                </Checkbox.Control>
            </Checkbox.Content>
        </Checkbox>
    );
}
