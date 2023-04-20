import { useEffect, useRef, useState } from "react"
import { invoke } from "@tauri-apps/api/tauri"

import { getValueFromRef, setValueToRef } from "./utils/react-utils"

type DownloadRequest = {
    url: string
    output_dir: string
}

const App = () => {
    const urlRef = useRef<HTMLInputElement | null>(null)
    const outputDirRef = useRef<HTMLInputElement | null>(null)

    const [showMessage, setShowMessage] = useState(false)
    const [message, setMessage] = useState("")
    const [isErrorMessage, setIsErrorMessage] = useState(false)

    const handleShowMessage = (msg: string) => {
        setMessage(msg)
        setShowMessage(true)
        setTimeout(() => {
            setMessage("")
            setShowMessage(false)
            setIsErrorMessage(false)
        }, 2500)
    }

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()

        const url = getValueFromRef(urlRef)
        setValueToRef(urlRef, "")

        const outputDir = getValueFromRef(outputDirRef)

        if (url === "" || outputDir === "") {
            setIsErrorMessage(true)
            handleShowMessage("Url and Output Dir are required")
            return
        }

        localStorage.setItem("last_output_dir", outputDir)

        // Always go for snake case when invoking Rust
        const resultMessage = (await invoke("download_video", {
            download_request: { url, output_dir: outputDir } as DownloadRequest
        })) as string

        handleShowMessage(resultMessage)
    }

    useEffect(() => {
        const lastOutputDir = localStorage.getItem("last_output_dir")
        if (lastOutputDir) {
            setValueToRef(outputDirRef, lastOutputDir)
        }
    }, [])

    return (
        <>
            {showMessage && (
                <div
                    onClick={() => setShowMessage(false)}
                    className={`message-container ${isErrorMessage && "error"}`}
                >
                    {message}
                </div>
            )}

            <div className="page-container">
                <div className="page-title">YTMusic Downloader</div>

                <form onSubmit={handleSubmit}>
                    <div className="form-group">
                        <label>URL</label>
                        <input type="text" ref={urlRef} required />
                    </div>
                    <div className="form-group">
                        <label>OutputDir</label>
                        <input type="text" ref={outputDirRef} required />
                    </div>
                    <div className="form-group">
                        <button type="submit">Download</button>
                    </div>
                </form>
            </div>
        </>
    )
}

export default App
