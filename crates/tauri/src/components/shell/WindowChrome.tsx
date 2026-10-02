import {Button, ButtonGroup} from "@heroui/react";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {Icon} from "@iconify-icon/react";

// Platform detection at module load, read synchronously from
// `navigator.userAgent` (works inside both WKWebView and WebKitGTK without
// an async `@tauri-apps/plugin-os` call).
//
// - macOS: `titleBarStyle: "Overlay"` (see tauri.macos.conf.json) draws the
//   native floating traffic lights overlaid on our content, so we hide the
//   HTML min/max/close and reserve space on the left for them.
// - Linux: `decorations: true` (see tauri.linux.conf.json) asks the window
//   manager to draw a native titlebar strip above the webview, which already
//   has its own min/max/close, so our HTML buttons are hidden to avoid
//   duplicating them.
// - Windows: decorations stay false, so the HTML buttons remain the only
//   window controls.
const IS_MACOS: boolean = typeof navigator !== "undefined"
    && /macintosh|mac os x/i.test(navigator.userAgent);
const IS_LINUX: boolean = typeof navigator !== "undefined"
    && !IS_MACOS
    && /linux/i.test(navigator.userAgent);
const HAS_NATIVE_CHROME: boolean = IS_MACOS || IS_LINUX;

// App-level titlebar: drag region, title, theme toggle and (on platforms
// without native decorations) the functional window controls.
export default function WindowChrome()
{
    const appWindow = getCurrentWindow();
    return (
        <div
            className={
                "flex flex-row h-8 backdrop-blur-sm sticky top-0 w-full z-51 backdrop-saturate-150 select-none bg-gray-100"
                + (IS_MACOS ? " pl-20" : "")
            }
            data-tauri-drag-region=""
        >
            <div className={"flex flex-row"}>
                <p className={"mx-2 mt-1 text-lg font-bold select-none"} data-tauri-drag-region="">Tauri Todo App</p>
            </div>
            <div className={"flex flex-row ml-auto"}>
                <ButtonGroup className={"h-8"} variant={"tertiary"}>
                    {!HAS_NATIVE_CHROME && (
                        <>
                            <Button key={"application-minimize-window-button"} variant={"tertiary"} className={"min-w-0 h-8 rounded-none text-[1rem] bg-transparent hover:bg-gray-200"} onPress={() => appWindow.minimize()}><Icon icon="material-symbols:minimize-rounded"/></Button>
                            <Button key={"application-maximize-window-button"} variant={"tertiary"} className={"min-w-0 h-8 rounded-none text-[.7rem] bg-transparent hover:bg-gray-200"} onPress={() => appWindow.toggleMaximize()}><Icon icon="material-symbols:square-outline-rounded"/></Button>
                            <Button key={"application-close-window-button"} variant={"danger-soft"} className={"min-w-0 h-8 rounded-none text-[1rem] bg-transparent hover:bg-danger hover:text-white"} onPress={() => appWindow.close()}><Icon icon="material-symbols:close-rounded"/></Button>
                        </>
                    )}
                </ButtonGroup>
            </div>
        </div>
    );
}

