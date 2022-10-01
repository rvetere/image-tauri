import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Image from "next/image";
import reactLogo from "../assets/react.svg";
import tauriLogo from "../assets/tauri.svg";
import nextLogo from "../assets/next.svg";
import Link from "next/link";

type BootPayload = {
  drives: string[];
};

function App() {
  const [bootPayload, setBootPayload] = useState<BootPayload>();
  const [dirs, setDirs] = useState<string[]>();
  const [cnt, setCnt] = useState<number>(0);

  useEffect(() => {
    invoke("start", { name: "bla" }).then((payload: BootPayload) => {
      setBootPayload(payload);
    });
  }, []);

  const handleScanDrive = (index) => () => {
    invoke("change_drive", { chgNum: index }).then((result) => {
      console.log({ result });
      invoke("scan_dir").then((_dirs: string[]) => {
        setDirs(_dirs);
        invoke("count_sub_dir").then((_cnt: number) => {
          setCnt(_cnt);
        });
      });
    });
  };

  return (
    <div className="container">
      <div className="info">
        {bootPayload &&
          bootPayload.drives.map((drive, index) => (
            <button onClick={handleScanDrive(index)}>{drive}</button>
          ))}
      </div>
      <div className="imageview">
        <img className="img-preview" src="" />
      </div>
      <div className="list">
        {dirs && dirs.map((dir) => <button>{dir}</button>)}
      </div>
    </div>
  );
}

export default App;
