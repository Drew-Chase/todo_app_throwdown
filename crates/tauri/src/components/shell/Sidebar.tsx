import {NavLink} from "react-router-dom";
import {Tooltip} from "@heroui/react";
import {Icon} from "@iconify-icon/react";

// Slim icon rail for app-shell navigation. Add entries here as pages are
// added — each maps a route to an Iconify icon and a tooltip label.
const items: { path: string; icon: string; label: string }[] = [
    {path: "/", icon: "material-symbols:home-rounded", label: "Home"}
];

export default function Sidebar()
{
    return (
        <div className="flex-shrink-0 flex flex-col items-center border-r border-separator bg-background w-[64px] py-4 gap-1.5">
            {items.map(item => (
                <Tooltip key={item.path} delay={250}>
                    <Tooltip.Trigger>
                        <NavLink
                            to={item.path}
                            end
                            className={({isActive}) =>
                                "flex items-center justify-center w-10 h-10 rounded-lg text-[1.25rem] transition-colors"
                                + (isActive
                                    ? " bg-accent/10 text-accent"
                                    : " text-foreground/60 hover:bg-foreground/5 hover:text-foreground")
                            }
                        >
                            <Icon icon={item.icon}/>
                        </NavLink>
                    </Tooltip.Trigger>
                    <Tooltip.Content placement="right">{item.label}</Tooltip.Content>
                </Tooltip>
            ))}

            <div className="flex-1"/>

            <Tooltip delay={250}>
                <Tooltip.Trigger>
                    <NavLink
                        to="/settings"
                        className={({isActive}) =>
                            "flex items-center justify-center w-10 h-10 rounded-lg text-[1.25rem] transition-colors"
                            + (isActive
                                ? " bg-accent/10 text-accent"
                                : " text-foreground/60 hover:bg-foreground/5 hover:text-foreground")
                        }
                    >
                        <Icon icon="material-symbols:settings-rounded"/>
                    </NavLink>
                </Tooltip.Trigger>
                <Tooltip.Content placement="right">Settings</Tooltip.Content>
            </Tooltip>
        </div>
    );
}
