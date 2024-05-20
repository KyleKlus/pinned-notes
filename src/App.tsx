import { Pin, PinOff, Plus, Trash } from "lucide-react";
import "./App.css";

import { appWindow } from '@tauri-apps/api/window';
import { useEffect, useState } from "react";
import { UnlistenFn } from "@tauri-apps/api/event";

const MARGIN: number = 8;

function getRandomHslColor() {
  const h = Math.floor(Math.random() * 360);
  const s = 40;
  const l = 90;
  return `hsl(${h}, ${s}%, ${l}%)`;
}

function App() {
  const [isLoaded, setIsLoaded] = useState(false);
  const [isPinned, setIsPinned] = useState(false);
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
      <div
        className={['header'].join(' ')}
        onMouseDown={() => {

        }}
        onMouseMove={() => {

        }}
        onMouseUp={() => {

        }}
      >
        <div className={['leftSide'].join(' ')}>
          <button className={['iconBtn'].join(' ')} onClick={() => { setIsPinned(!isPinned) }}>
            {!isPinned
              ? <Pin width={16} height={16} />
              : <PinOff width={16} height={16} />
            }
          </button>
        </div>
        <div className={['rightSide'].join(' ')}>
          <button className={['iconBtn'].join(' ')}><Plus width={16} height={16} /></button>
          <button className={['iconBtn'].join(' ')}><Trash width={16} height={16} /></button>
        </div>
      </div>
      <textarea className={['text'].join(' ')}></textarea>
    </div>
  );
}

export default App;
