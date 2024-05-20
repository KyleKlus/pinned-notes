import React from "react";
import ReactDOM from "react-dom/client";
import Note from "./Note";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Note />
  </React.StrictMode>,
);
