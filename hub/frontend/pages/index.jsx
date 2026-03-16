import useSWR from "swr"
import axios from "axios"
import Dashboard from "../components/Dashboard"

const fetcher = (url) => axios.get(url).then(r=>r.data)

export default function Home(){
  const { data } = useSWR("/api/proxy/health", fetcher, {refreshInterval: 5000})
  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-4">Aurora Control Hub</h1>
      <Dashboard status={data}/>
    </div>
  )
}
