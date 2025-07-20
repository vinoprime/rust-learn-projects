import { Channel, invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import "./App.css";

const App: React.FC = () => {
  const [clipboardItems, setClipboardItems] = useState<string[]>([]);
  const [filter, setFilter] = useState<number>(5);
  const [status, setStatus] = useState<string>();

  const fetchClipboardHistory = async (n: number) => {
    try {
      const items: string[] = await invoke("load_last_n_entries", { n });
      setClipboardItems(items);
      setStatus(items.length > 0 ? "" : "No clipboard history found!");
    } catch (error) {
      console.log("Error fetching clipboard history", error);
      setStatus("Failed to load clipboard history");
    }
  };

  const copyClipboard = async (data: string) => {
    try {
      await invoke("copy", { data });
    } catch (error) {
      console.log("Error copying clipboard history", error);
    }
  };

  const wipeAllClipboardHistory = async () => {
    try {
      await invoke("wipe_all");
      setClipboardItems([]);
    } catch (error) {
      console.log("Error wiping clipboard history", error);
    }
  };

  useEffect(() => {
    const initClipboard = async () => {
      try {
        const onEvent = new Channel<string>();

        onEvent.onmessage = (msg: string) => {
          console.log("Clipboard updated", msg);
          setClipboardItems((prevItems) => [msg, ...prevItems]);
        };

        await invoke("init", { onEvent });
      } catch (error) {
        console.log("Error init  clipboard monitoring", error);
      }
    };

    initClipboard();
    fetchClipboardHistory(filter);

    return () => {
      console.log("Cleaning up on unmount");
    };
  }, [filter]);

  return (
    <>
      <div className="app">
        <header className="header">
          <h1>ClipBoard</h1>
          <p>Manage clipboard history with ease</p>
        </header>
        <main className="app-main">
          <div className="controls">
            <div className="filter-container">
              <label htmlFor={"filter"}>Show last:</label>
              <select
                id="filter"
                value={filter}
                onChange={(e) => setFilter(Number(e.target.value))}
              >
                <option value={5}>5</option>
                <option value={10}>10</option>
                <option value={20}>20</option>
                <option value={50}>50</option>
              </select>
            </div>
            <button className="wipe-button" onClick={wipeAllClipboardHistory}>
              Wipe All
            </button>
          </div>
          {/* below is conditional ui */}
          {status && <p className="status">{status}</p>}
          <ul className="clipboard-list">
            {clipboardItems.map((item: string, index: number) => (
              <li key={index} className="item-text">
                <span>{item}</span>
                <button
                  className="copy-button"
                  onClick={() => copyClipboard(item)}
                >
                  Copy
                </button>
              </li>
            ))}
          </ul>
        </main>
      </div>
    </>
  );
};

export default App;
