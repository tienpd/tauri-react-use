import { Staff } from "@/src-tauri/bindings/Staff"
import { invoke } from "@tauri-apps/api/core"
import { use } from "react"

export default function Staffs() {
  const staffs = use(invoke<Staff[]>('get_staff_list'))
  
  // THIS IS WILL WORKS ON WINDOWS
  // const [staffs, setStaffs] = useState<Staff[]>([])

  // useEffect(() => {
  //   const fetchStaffs = async () => {
  //     const staffs = await invoke<Staff[]>('get_staff_list')
  //     setStaffs(staffs)
  //   }
  //   fetchStaffs()
  // }, [])

  return (
    <div>
      <h1>Staffs</h1>
      <div>
        {staffs.map((staff: Staff) => {
          return (
            <div key={staff.name}>
              <img src={staff.avatar} alt={staff.name} />
              <p>{staff.name}</p>
              <p>{staff.total_appointments}</p>
            </div>
          )
        })}
      </div>
    </div>
  )
}