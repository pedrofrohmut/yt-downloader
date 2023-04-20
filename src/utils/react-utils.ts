export const getValueFromRef = (ref: React.RefObject<HTMLInputElement>) => {
    if (!ref || !ref.current || ref.current.value === "") {
        return ""
    } else {
        return ref.current.value
    }
}

export const setValueToRef = (ref: React.RefObject<HTMLInputElement>, value: string) => {
    if (ref && ref.current) {
        ref.current.value = value
    }
}
