import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import IconPlay from "./assets/play.png"
import IconPause from "./assets/pause.png"


function App() {
  let seconds = 30
  let minutes = 1

  let [timer, setTimer] = useState(getFormatTimer())
  
  function getFormatTimer(){
    return `${minutes.toString().padStart(2, "0")}:${seconds.toString().padStart(2, "0")}`
  }

  useEffect(()=> {
    const timerId = setInterval(()=> {
      if (seconds != 0) {
        seconds -= 1;
        setTimer(getFormatTimer());
      } else if (seconds == 0 && minutes != 0) {
        seconds = 59;
        minutes -= 1;
        setTimer(getFormatTimer());
      } else if (seconds == 0 && minutes == 0) {
        return 
      }
    }, 1000);
    return () => clearInterval(timerId)
  }, [])

  async function handler_pause(event){
    event.preventDefault()
  }

  return (
    <div className="container">
      <div className="timer">
        <span>{timer}</span>
      </div>
      <div className="amount">
        <div></div>
        <div></div>
        <div></div>
      </div>
      <div className="actions">
        <button>
          <img src={IconPlay} alt="Run" />
        </button>
        <button>
          <img src={IconPause} alt="Pause" />
        </button>
      </div>
    </div>
  );
}

export default App;
