import { toast } from "sonner"

export interface JsonViewerProps {
  jsonData: any;
}

export const JsonViewer = (props: { data: JsonViewerProps }) => {
  const onClick = () => {
    navigator.clipboard.writeText(JSON.stringify(props.data, null, 2));
    toast.success("Copied json to clipboard!");
  }
  return <>
    <div
      className="flex-1 p-4 overflow-auto"
    >
      <pre
        className="bg-white p-4 border rounded shadow"
        onClick={onClick}
      >
        <code className="whitespace-pre-wrap">
          {JSON.stringify(props.data, null, 2)}
        </code>
      </pre>
    </div>
  </>;
};
