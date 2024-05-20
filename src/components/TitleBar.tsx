import { Pin, PinOff, Plus, Trash } from "lucide-react";
import { useState } from "react";
import './TitleBar.css';
import { appWindow } from "@tauri-apps/api/window";

function TitleBar() {
    const [isPinned, setIsPinned] = useState(false);

    return (
        <div
            data-tauri-drag-region
            className={['titleBar'].join(' ')}
            onDragStart={async () => { await appWindow.startDragging() }}
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
    );
}

export default TitleBar;