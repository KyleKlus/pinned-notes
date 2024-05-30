import "./Note.css";

import { appWindow } from '@tauri-apps/api/window';
import { useEffect, useRef, useState } from "react";
import { UnlistenFn } from "@tauri-apps/api/event";
import TitleBar from "./components/TitleBar";
import INote from "./interfaces/INote";
import { invoke } from "@tauri-apps/api";

const MARGIN: number = 8;

function App() {
  const [isLoaded, setIsLoaded] = useState(false);
  const [pinned, setPinned] = useState(false);
  const [uuid, setUUID] = useState('');
  const [text, setText] = useState('');
  const [color, setColor] = useState('');
  const [position, setPosition] = useState<{ x: number, y: number }>({ x: 0, y: 0 });
  const [size, setSize] = useState<{ width: number, height: number }>({ width: 0, height: 0 });

  const lastSaveCall = useRef<Date | undefined>(undefined);


  useEffect(() => {
    if (isLoaded) { return; }
    loadNoteData();
  })

  useEffect(() => {
    // Listen for save requests
    if (lastSaveCall.current === undefined) { return; }

    const saveIntervall = setInterval(() => {
      // Check if last save call is older than 5s
      const now = new Date();
      if (
        lastSaveCall.current === undefined ||
        now.getTime() - lastSaveCall.current.getTime() < 5000
      ) { return; }

      // Save note
      const note: INote = exportStateToNote();
      invoke('save_note', { note: note });

      // Reset last save call
      lastSaveCall.current = undefined;
      clearInterval(saveIntervall)
    }, 1000);
    return () => clearInterval(saveIntervall);
  })

  useEffect(() => {
    // Listen for resizes
    let unlisten: UnlistenFn | undefined = undefined;
    const listen = async () => {
      unlisten = await appWindow.onResized(async ({ payload: size }) => {
        // Save note 5s after the last onResize
        const now = new Date();
        lastSaveCall.current = now;

        // Update size
        const scaleFactor = await appWindow.scaleFactor();
        setSize({ width: size.toLogical(scaleFactor).width - MARGIN, height: size.toLogical(scaleFactor).height - MARGIN });
      });
    };

    listen();

    return () => unlisten && unlisten();
  })

  useEffect(() => {
    // Listen for dragging
    let unlisten: UnlistenFn | undefined = undefined;
    const listen = async () => {
      unlisten = await appWindow.onMoved(async ({ payload: position }) => {
        // Save note 5s after the last onMove
        const now = new Date();
        lastSaveCall.current = now;

        // Update position
        setPosition({ x: position.x, y: position.y });
      });
    };

    listen();

    return () => unlisten && unlisten();
  })

  async function loadNoteData() {
    if (isLoaded) { return; }

    const uuid: string = appWindow.label;
    const note: INote = (await invoke('load_note', { uuid: uuid })) as INote;

    setPinned(note.pinned);
    setUUID(note.uuid);
    setText(note.text);
    setColor(note.color);
    setSize({ width: note.width, height: note.height });
    setPosition({ x: note.x, y: note.y });

    setIsLoaded(true);
  }

  function exportStateToNote(): INote {
    const note: INote = {
      pinned: pinned,
      uuid: uuid,
      text: text,
      color: color,
      x: position.x,
      y: position.y,
      width: size.width,
      height: size.height
    };

    return note;
  }

  return (
    <>
      {isLoaded
        ? <div
          className={['note'].join(' ')}
          style={{ backgroundColor: color, width: `${size.width}px`, height: `${size.height}px` }}
        >
          <TitleBar
            pinned={pinned}
            uuid={uuid}
            setIsPinned={async (state: boolean) => {
              setPinned(state);
              const updatedNote = exportStateToNote();
              await invoke('save_note', { note: updatedNote });
            }}
          />
          <textarea
            className={['text'].join(' ')}
            value={text}
            onChange={(e) => {
              setText(e.target.value);
              const updatedNote = exportStateToNote();
              invoke('save_note', { note: updatedNote });
            }}
          />
        </div >
        : <div />
      }
    </>
  );
}

export default App;
