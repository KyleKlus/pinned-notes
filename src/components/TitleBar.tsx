import { Pin, PinOff, Plus, Trash } from "lucide-react";
import './TitleBar.css';
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from '@tauri-apps/api/tauri';
import INote from "../interfaces/INote";

function TitleBar(props: {
    note: INote | undefined,
    setIsPinned: (state: boolean) => void
}) {
    return (
        <div
            data-tauri-drag-region={props.note?.pinned ? null : true}
            className={['titleBar'].join(' ')}
            onDragStart={async () => {
                if (props.note?.pinned) { return; }
                await appWindow.startDragging()
            }}
        >
            <div className={['leftSide'].join(' ')}>
                <button className={['iconBtn'].join(' ')} onClick={() => {
                    props.setIsPinned(!props.note?.pinned)
                }}>
                    {!props.note?.pinned
                        ? <Pin width={16} height={16} />
                        : <PinOff width={16} height={16} />
                    }
                </button>
            </div>
            <div className={['rightSide'].join(' ')}>
                <button
                    className={['iconBtn'].join(' ')}
                    onMouseDown={() => {
                        invoke('create_new_note_from_note');
                    }}
                >
                    <Plus width={16} height={16} />
                </button>
                <button
                    className={['iconBtn'].join(' ')}
                    onMouseDown={() => {
                        invoke('delete_note', { uuid: props.note?.uuid });
                    }}
                >
                    <Trash width={16} height={16} />
                </button>
            </div>
        </div>
    );
}

export default TitleBar;