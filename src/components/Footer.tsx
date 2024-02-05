import Image from "next/image"
import { FC } from "react"

const Footer: FC = () => {
  return (
    <footer className="flex flex-1 flex-grow-0 items-center justify-center border-t border-gray-200 py-4">
      <div>
        <a
          href="https://tauri.app/"
          target="_blank"
          rel="noopener noreferrer"
          className="flex flex-grow items-center justify-center p-4"
        >
          Dragan Image Color Picker Powered by{" "}
          <span className="ml-2 h-6">
            <Image
              src="/tauri_logo_light.svg"
              alt="Vercel Logo"
              height={24}
              width={78}
            />
          </span>
        </a>
      </div>
    </footer>
  )
}

export default Footer
