export const getValueFromRef = (ref: React.RefObject<HTMLInputElement>) =>
    !ref || !ref.current || ref.current.value === "" ? "" : ref.current.value
