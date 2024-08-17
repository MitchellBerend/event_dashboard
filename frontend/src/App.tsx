import { useEffect, useState } from "react";
import { Event, EventTable } from "./EventTable"
import { JsonViewer } from "./JsonViewer";
import { Toaster } from "./components/ui/sonner";
import eventService from "./services/events.service";

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
        const events = await eventService.getEvents();
        setData(events);
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
        const event = await eventService.getEvent(reference);
        setDatum(event);
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
