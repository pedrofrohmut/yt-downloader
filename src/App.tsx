import { useRef, useState } from "react"

import { getValueFromRef } from "./utils/react-utils"

const App = () => {
    const artistRef = useRef<HTMLInputElement>(null)

    const [typingTimeout, setTypingTimeout] = useState<NodeJS.Timeout | null>(null)

    const search = (text: string) => {
        console.log("SEARCH: " + text)
    }

    const delayedSearch = () => {
        if (typingTimeout) clearTimeout(typingTimeout)

        const searchText = getValueFromRef(artistRef)
        if (searchText === "") return

        const newTimeout = setTimeout(() => search(searchText), 700)
        setTypingTimeout(newTimeout)
    }

    return (
        <div className="page-container">
            <form>
                <div className="form-group">
                    <label>Artist</label>
                    <input type="text" ref={artistRef} onKeyUp={delayedSearch} />
                </div>
            </form>
        </div>
    )
}

export default App
