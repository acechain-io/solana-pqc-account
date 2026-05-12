import React from "react";
import ReactDOM from "react-dom/client";
import { Buffer } from "buffer";
import App from "./App";

// Polyfill Buffer for browser
(window as any).Buffer = Buffer;

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
