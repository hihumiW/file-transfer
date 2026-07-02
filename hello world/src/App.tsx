import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

type TaskProgressPayload = {
  progress : number;
}

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  const [deviceName, setDeviceName] = useState("");
  const [deviceNameValidationMsg, setDeviceNameValidationMsg] = useState("");
  async function validateDeviceName() {
    try {
      const validationMsg = (await invoke("validate_device_name", {
        deviceName: deviceName,
      })) as string;
      setDeviceNameValidationMsg(validationMsg);
    } catch (e : string | any) {
      console.dir(e);
      setDeviceNameValidationMsg(
        `An error occurred while validating the device name : ${e}`,
      );
    }
  }

  async function resolveDeviceName(){
    const resolvedDeviceName = await invoke('resolve_device_name', {
      customName : deviceName,
      defaultName : ""
    });
    console.log(resolvedDeviceName);
  }


  const [progress, setProgress] = useState(0);
  useEffect(() => {
    const unlistenPromise  = listen<TaskProgressPayload>('task-progress', (event) => {
      console.log(event.payload, 'event payload');
      setProgress(event.payload.progress);
    });

    const taskDoneListenPromise = listen('task-done', () => {
      console.log('done!!!');
    });

    return () => {
      unlistenPromise.then(unlistenFn => unlistenFn());
      taskDoneListenPromise.then(unlistenFn => unlistenFn());
    }
  }, []);

  function startFakeTask(){
    invoke('start_fake_task', {
      taskName : "task1"
    })
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>
      <p>progress : {progress}%</p>
      <button onClick={startFakeTask}>start fake task</button>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          validateDeviceName();
        }}
      >
        <input
          onChange={(e) => setDeviceName(e.currentTarget.value)}
          placeholder="Enter a device name..."
        />
        <button type="submit">validate deviceName</button>
        <button type="button" onClick={resolveDeviceName}>resolve device name</button>
      </form>
      <p>{deviceNameValidationMsg}</p>
    </main>
  );
}
 
export default App;
