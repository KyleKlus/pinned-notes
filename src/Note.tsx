import "./Note.css";

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
    // listen for rescales

    let unlisten: UnlistenFn | undefined = undefined;
    const listen = async () => {
      unlisten = await appWindow.onScaleChanged(async ({ payload: scale }) => {
        console.log(scale)
      });
    };

    listen();

    return () => unlisten && unlisten();
  })

  useEffect(() => {
    // Listen for resizes
    let unlisten: UnlistenFn | undefined = undefined;
    const listen = async () => {
      unlisten = await appWindow.onResized(async ({ payload: size }) => {
        const scaleFactor = await appWindow.scaleFactor();

        setWidth(size.toLogical(scaleFactor).width - MARGIN);
        setHeight(size.toLogical(scaleFactor).height - MARGIN);
      });
    };

    listen();

    return () => unlisten && unlisten();
  })

  async function loadNoteData() {
    if (isLoaded) { return; }

    const size = await appWindow.innerSize();
    const scaleFactor = await appWindow.scaleFactor();

    setWidth(size.toLogical(scaleFactor).width - MARGIN);
    setHeight(size.toLogical(scaleFactor).height - MARGIN);

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
