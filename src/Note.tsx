import "./Note.css";

import { appWindow } from '@tauri-apps/api/window';
import { useEffect, useState } from "react";
import { UnlistenFn } from "@tauri-apps/api/event";
import TitleBar from "./components/TitleBar";
import INote from "./interfaces/INote";
import { invoke } from "@tauri-apps/api";

const MARGIN: number = 8;

function App() {
  const [isLoaded, setIsLoaded] = useState(false);
  const [note, setNote] = useState<INote | undefined>(undefined);
  const [width, setWidth] = useState(0);
  const [height, setHeight] = useState(0);

  useEffect(() => {
    if (isLoaded) { return; }
    loadNoteData();
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

  useEffect(() => {
    // Listen for dragging
    let unlisten: UnlistenFn | undefined = undefined;
    const listen = async () => {
      unlisten = await appWindow.onMoved(async ({ payload: position }) => {
        if (note === undefined) { return; }
        const updatedNote: INote = {
          ...note,
          x: position.x,
          y: position.y
        };

        invoke('save_note', { note: updatedNote });
        setNote(updatedNote);
      });
    };

    listen();

    return () => unlisten && unlisten();
  })

  async function loadNoteData() {
    if (isLoaded) { return; }

    const uuid: string = appWindow.label;

    const note: INote = (await invoke('load_note', { uuid: uuid })) as INote;

    const size = await appWindow.innerSize();
    const scaleFactor = await appWindow.scaleFactor();

    setWidth(size.toLogical(scaleFactor).width - MARGIN);
    setHeight(size.toLogical(scaleFactor).height - MARGIN);
    setNote(note);

    setIsLoaded(true);
  }

  return (
    <>
      {isLoaded
        ? <div
          className={['note'].join(' ')}
          style={{ backgroundColor: note?.color, width: `${width}px`, height: `${height}px` }
          }
        >
          <TitleBar
            note={note}
            setIsPinned={async (state: boolean) => {
              if (note === undefined) { return; }
              const updatedNote: INote = { ...note, pinned: state };
              await invoke('save_note', { note: updatedNote });
              setNote(updatedNote);
            }}
          />
          <textarea
            className={['text'].join(' ')}
            value={note?.text}
            onChange={(e) => {
              if (note === undefined) { return; }
              const updatedNote: INote = { ...note, text: e.target.value };
              invoke('save_note', { note: updatedNote });
              setNote(updatedNote);
            }}
          />
        </div >
        : <div />
      }
    </>
  );
}

export default App;
