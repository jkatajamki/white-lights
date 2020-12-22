import theme from "../../theme/theme"

export interface TextInputProps {
  id: string
  labelText: string
  onChange(event: React.ChangeEvent<HTMLInputElement>): void
  autoCompleteType?: string
  value?: string
  isPassword?: boolean
}

const TextInput = ({
  id,
  labelText,
  onChange,
  autoCompleteType,
  value,
  isPassword,
}: TextInputProps): JSX.Element => (
  <>
    <label htmlFor={id}>{labelText}</label>
    <input
      id={id}
      type={isPassword ? "password" : "text"}
      onChange={onChange}
      autoComplete={autoCompleteType}
      value={value}
    />
    <style jsx>{`
      input {
        background: ${theme.colors.background.tertiary};
        color: ${theme.colors.text.primary};
        height: 2rem;
        font-size: 1.25rem;
        outline-color: ${theme.colors.text.tertiary};
        width: 100%;
        border 0.15rem solid ${theme.colors.text.secondary};
        border-radius: 0.1rem;
        margin: 0.3rem 0 0.7rem;
      }

      label {
        color: ${theme.colors.text.secondary};
      }
    `}</style>
  </>
)

export default TextInput
