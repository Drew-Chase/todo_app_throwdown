import React from "react";
import {BrowserRouter, Route, Routes} from "react-router-dom";
import ReactDOM from "react-dom/client";

import "./css/index.css";
import AppShell from "./components/shell/AppShell.tsx";
import Home from "./pages/Home.tsx";
import Settings from "./pages/Settings.tsx";
import NotFound from "./pages/NotFound.tsx";
import {ThemeProvider} from "./providers/ThemeProvider.tsx";
import {Toast} from "@heroui/react";
import {attachConsoleToTracing} from "./util/logger.ts";

// Route all console output and uncaught errors through the Rust tracing
// pipeline so frontend logs land in the same rolling log files as native logs.
attachConsoleToTracing();

// Disable the browser's default right-click menu; it has no relevance
// inside a native desktop shell.
document.addEventListener("contextmenu", e => e.preventDefault());

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <BrowserRouter>
            <ThemeProvider>
                <Toast.Provider placement={"bottom end"}/>
                <Routes>
                    <Route element={<AppShell/>}>
                        <Route path="/" element={<Home/>}/>
                        <Route path="/settings" element={<Settings/>}/>
                        <Route path="*" element={<NotFound/>}/>
                    </Route>
                </Routes>
            </ThemeProvider>
        </BrowserRouter>
    </React.StrictMode>
);
