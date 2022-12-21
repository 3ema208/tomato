import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import IconPlay from "./assets/play.png"
import IconPause from "./assets/pause.png"
import IconTomato from "./assets/tomato2.png"


function App() {
  let [amountTomato, setAmountTomato] = useState([...Array(10).keys()])
  let [timer, setTimer] = useState("Unk")
  
  useEffect(
    ()=>{
      let intervalId = setInterval(async ()=>{
        try {
          let data = await invoke("get_tomato_period")
          setTimer(data)
        } catch (err) {
          console.log(err)
          setTimer("error")
        }
      }, 0)
      return () => clearInterval(intervalId);
    }, 
    []
    )

  const handle_start = async () => {
    await invoke("run_timer")
  }

  const handle_stop = async () => {
    await invoke("stop_timer")
  }

  return (
    <div className="container">
      <div className="timer">
        <span>{timer}</span>
      </div>
      <div className="amount">
        {
          amountTomato.map(el => {
            return <img key={el} src={IconTomato}></img>
          })
        }
      </div>
      <div className="actions">
        <button onClick={handle_start}>
          <img src={IconPlay} alt="Run" />
        </button>
        <button onClick={handle_stop}>
          <img src={IconPause} alt="Pause" />
        </button>
      </div>
    </div>
  );
}

export default App;
