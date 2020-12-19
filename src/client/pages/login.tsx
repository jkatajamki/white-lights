import { useState } from "react"
import TextInput from "../components/form/TextInput"

const LoginPage = () => {
  const [email, setEmail] = useState("")

  const onChangeEmail = (event: React.ChangeEvent<HTMLInputElement>) => {
    const {
      target: { value },
    } = event

    setEmail(value)
  }

  return (
    <>
      <h1>Sign in to White Lights</h1>

      <TextInput
        id="email"
        labelText="Enter your email address here"
        onChange={onChangeEmail}
        autoCompleteType="email"
        value={email}
      />
    </>
  )
}

export default LoginPage
