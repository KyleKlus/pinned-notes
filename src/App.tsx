import "./App.css";

import { appWindow } from '@tauri-apps/api/window';
import { useEffect, useState } from "react";
import { UnlistenFn } from "@tauri-apps/api/event";
import TitleBar from "./components/TitleBar";

const MARGIN: number = 8;

function getRandomHslColor() {
  const h = Math.floor(Math.random() * 360);
  const s = 40;
  const l = 90;
  return `hsl(${h}, ${s}%, ${l}%)`;
}

function App() {
  const [isLoaded, setIsLoaded] = useState(false);
  const [width, setWidth] = useState(0);
  const [height, setHeight] = useState(0);
  const [color, setColor] = useState(getRandomHslColor());

  useEffect(() => {
    if (isLoaded) { return; }
    loadNoteData();
  })

  useEffect(() => {
    let unlisten: UnlistenFn | undefined = undefined;
    const listen = async () => {
      unlisten = await appWindow.onResized(({ payload: size }) => {
        setWidth(size.width - MARGIN);
        setHeight(size.height - MARGIN);
      });
    };

    listen();

    return () => unlisten && unlisten();
  })

  async function loadNoteData() {
    if (isLoaded) { return; }

    const size = await appWindow.innerSize();
    setWidth(size.width - MARGIN);
    setHeight(size.height - MARGIN);

    setIsLoaded(true);
  }

  return (
    <div className={['note'].join(' ')} style={{ backgroundColor: color, width: `${width}px`, height: `${height}px` }}>
      <TitleBar />
      <textarea className={['text'].join(' ')}></textarea>
    </div>
  );
}

export default App;
