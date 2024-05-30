import { Pin, PinOff, Plus, Trash } from "lucide-react";
import './TitleBar.css';
import { invoke } from '@tauri-apps/api/tauri';

function TitleBar(props: {
    uuid: string,
    pinned: boolean,
    setIsPinned: (state: boolean) => void
}) {
    return (
        <div
            data-tauri-drag-region={props.pinned ? null : true}
            className={['titleBar'].join(' ')}
        >
            <div className={['leftSide'].join(' ')}>
                <button className={['iconBtn'].join(' ')} onClick={() => {
                    props.setIsPinned(!props.pinned)
                }}>
                    {!props.pinned
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
                        invoke('delete_note', { uuid: props.uuid });
                    }}
                >
                    <Trash width={16} height={16} />
                </button>
            </div>
        </div>
    );
}

export default TitleBar;