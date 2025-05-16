import { Suspense } from "react"
import { createFileRoute } from "@tanstack/react-router"
import Staffs from "./-components/staffs"

export const Route = createFileRoute("/")({
  component: Index,
})

function Index() {
  return (
    <div className="p-2">
      <h3>Welcome Home!</h3>
      <Suspense
        fallback={
          <div className="flex h-screen w-full items-center justify-center bg-red-700">
            Loading...
          </div>
        }
      >
        <Staffs />
      </Suspense>
    </div>
  )
}
