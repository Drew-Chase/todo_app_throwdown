import React from "react";
import {BrowserRouter, Route, Routes} from "react-router-dom";
import ReactDOM from "react-dom/client";

import "./css/index.css";
import AppShell from "./components/shell/AppShell.tsx";
import Home from "./pages/Home.tsx";
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
            <Routes>
                <Route element={<AppShell/>}>
                    <Route path="/" element={<Home/>}/>
                </Route>
            </Routes>
        </BrowserRouter>
    </React.StrictMode>
);
