export interface TextInputProps {
  id: string
  labelText: string
  onChange(event: React.ChangeEvent<HTMLInputElement>): void
  autoCompleteType?: string
  value?: string
}

const TextInput = ({
  id,
  labelText,
  onChange,
  autoCompleteType,
  value,
}: TextInputProps): JSX.Element => (
  <>
    <label htmlFor={id}>{labelText}</label>
    <input
      id={id}
      type="text"
      onChange={onChange}
      autoComplete={autoCompleteType}
      value={value}
    />
  </>
)

export default TextInput
