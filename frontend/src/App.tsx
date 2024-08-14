import { useEffect, useState } from "react";
import { Event, EventTable } from "./EventTable"
import { JsonViewer } from "./JsonViewer";
import { Toaster } from "./components/ui/sonner";

function App() {
  const [data, setData] = useState<Array<Event>>([]);
  const [reference, setReference] = useState<string>();
  const [datum, setDatum] = useState<any>();

  const onClickDatum = (ref: any) => {
    setReference(ref);
  };

  useEffect(() => {
    async function getData() {
      try {
        const response = await fetch('http://192.168.1.171:8080/events');
        const result = await response.json();

        setData(result);

      } catch (error) {
        console.error('Error fetching data:', error);
      }
    }

    getData().then();
  }, []);

  useEffect(() => {
    async function getReferenceData() {
      try {
        if (!reference) {
          return;
        }
        const response = await fetch(`http://192.168.1.171:8080/event/${reference}`);
        const result = await response.json();
        setDatum(result);
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    }
    getReferenceData().then();
  }, [reference])

  return <>
    <div className="flex bg-gray-100 h-screen">
      <div
        className="flex-1 p-4 overflow-auto"
      >
        <EventTable className="bg-white p-4 border rounded shadow" onClickDatum={onClickDatum} events={data} />
      </div>
      <JsonViewer data={datum} />
    </div>
    <Toaster />
  </>
}

export default App
