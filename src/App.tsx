import { invoke } from "@tauri-apps/api/tauri";
import { appWindow, LogicalSize, PhysicalSize } from '@tauri-apps/api/window';
import "./App.css";
import { Plus } from "lucide-react";
import { useEffect } from "react";

function App() {
  useEffect(() => {
    correctDynamicWindowSize();
  });

  async function correctDynamicWindowSize() {
    const size = new PhysicalSize(50, 50);
    appWindow.setSize(size);
  }

  return (
    <button className='addBtn'>
      <Plus width={25} height={25}/>
    </button>
  );
}

export default App;
