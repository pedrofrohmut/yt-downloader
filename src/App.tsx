import { useEffect, useRef, useState } from "react"
import { invoke } from "@tauri-apps/api/tauri"

import { getValueFromRef, setValueToRef } from "./utils/react-utils"

// Use snake here to make it compatible with the rust backend
type DownloadRequest = {
    url: string
    output_dir: string
    audio_only: boolean
    file_name: string
}

type ResultMessage = {
    is_error: boolean
    message: string
}

const App = () => {
    const urlRef = useRef<HTMLInputElement | null>(null)
    const outputDirRef = useRef<HTMLInputElement | null>(null)
    const fileNameRef = useRef<HTMLInputElement | null>(null)

    const [audioOnly, setAudioOnly] = useState(true)

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
        const outputDir = getValueFromRef(outputDirRef)
        const fileName = getValueFromRef(fileNameRef)

        if (url === "" || outputDir === "" || fileName === "") {
            setIsErrorMessage(true)
            handleShowMessage("Url, Output Dir, FileName are required")
            return
        }

        localStorage.setItem("last_output_dir", outputDir)

        // Always go for snake case when invoking Rust
        const downloadRequest = {
            url,
            output_dir: outputDir,
            audio_only: audioOnly,
            file_name: fileName
        } as DownloadRequest

        const checkFileExistsResultMessage = (await invoke("check_file_exists", {
            download_request: downloadRequest
        })) as ResultMessage

        if (checkFileExistsResultMessage.is_error) {
            setIsErrorMessage(true)
            handleShowMessage(checkFileExistsResultMessage.message)
            return
        }

        const downloadResultMessage = (await invoke("download_video", {
            download_request: downloadRequest
        })) as string

        if (downloadResultMessage.includes("Error")) {
            setIsErrorMessage(true)
        } else {
            setValueToRef(urlRef, "")
        }
        handleShowMessage(downloadResultMessage)
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
                    {/* URL */}
                    <div className="form-group">
                        <label>URL</label>
                        <input type="text" ref={urlRef} required />
                    </div>

                    {/* Output Dir */}
                    <div className="form-group">
                        <label>OutputDir</label>
                        <input type="text" ref={outputDirRef} required />
                    </div>

                    {/* File Name */}
                    <div className="form-group">
                        <label>File Name</label>
                        <input type="text" ref={fileNameRef} required />
                    </div>

                    {/* Media Type */}
                    <div className="form-group">
                        <label>
                            <input
                                type="radio"
                                value="audio"
                                checked={audioOnly}
                                onChange={() => setAudioOnly(true)}
                            />
                            Audio Only
                        </label>

                        <label>
                            <input
                                type="radio"
                                value="video"
                                checked={!audioOnly}
                                onChange={() => setAudioOnly(false)}
                            />
                            Video
                        </label>
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
