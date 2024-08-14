import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"

export interface Event {
  reference: string,
  createdAt: Date,
}

export function EventTable(props: { className: string | undefined, events: Array<Event>, onClickDatum: (datum: any) => void }) {
  const userLocale = navigator.language || 'nl-NL';
  return <>
    <Table className={props.className}>
      <TableCaption>A list of past events</TableCaption>
      <TableHeader>
        <TableRow>
          <TableHead className="w-[100px]">Event reference</TableHead>
          <TableHead className="w-[200px]">Received At ({userLocale})</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {props.events.map((event, idx) => (
          <EventRow key={idx} event={event} locale={userLocale} onClickDatum={props.onClickDatum}  />
        ))}
      </TableBody>
    </Table>
  </>
}

function EventRow(props: { event: Event, locale: string, onClickDatum: (datum: any) => void }) {
  const timestamp = props.event.createdAt;
  const date = new Date(timestamp);

  // Using toLocaleString with options to customize the format
  const formattedDate = date.toLocaleString(props.locale, {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  });

  return <>
    <TableRow onClick={() => { props.onClickDatum(props.event.reference) }} key={props.event.reference}>
      <TableCell className="font-medium">{props.event.reference}</TableCell>
      <TableCell className="font-medium">{formattedDate}</TableCell>
    </TableRow >
  </>;
}
