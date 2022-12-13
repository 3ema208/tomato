﻿import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import IconPlay from "./assets/play.png"
import IconPause from "./assets/pause.png"


function App() {
  const [timer, setTimer] = useState("12:32");
  const [running, isRunning] = useState(false);
  
  async function handler_run() {
    console.log("run", running)
  }

  async function handler_pause(){
    console.log("pause", running)
  }

  return (
    <div className="container">
      <div className="timer">
        <span>{timer}</span>
      </div>
      <div className="amount">
        <div></div>
      </div>
      <div className="actions">
        <button onClick={handler_run}>
          <img src={IconPlay} alt="Run" />
        </button>
        <button onClick={handler_pause}>
          <img src={IconPause} alt="Pause" />
        </button>
      </div>
    </div>
  );
}

export default App;
