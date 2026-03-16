export default function Dashboard({status}){
  return (
    <div>
      <div className="grid grid-cols-4 gap-4">
        <div className="p-4 border rounded">Agents: {status?.agents ?? '—'}</div>
        <div className="p-4 border rounded">Tasks: {status?.tasks ?? '—'}</div>
        <div className="p-4 border rounded">Time: {status?.time ?? '—'}</div>
        <div className="p-4 border rounded">Status: {status?.status ?? '—'}</div>
      </div>
    </div>
  )
}
