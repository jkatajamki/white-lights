import { useState } from "react"
import Head from "next/head"
import TextInput from "../components/form/TextInput"
import FormContainer from "../components/form/FormContainer"

const LoginPage = () => {
  const [email, setEmail] = useState("")

  const [password, setPassword] = useState("")

  const onChangeEmail = (event: React.ChangeEvent<HTMLInputElement>) => {
    const {
      target: { value },
    } = event

    setEmail(value)
  }

  const onChangePassword = (event: React.ChangeEvent<HTMLInputElement>) => {
    const {
      target: { value },
    } = event

    setPassword(value)
  }

  return (
    <>
      <Head>
        <title>White Lights | Sign In</title>
      </Head>

      <section className="content flex-content">
        <FormContainer>
          <h1>Sign in to White Lights</h1>

          <TextInput
            id="email"
            labelText="Enter your email address here"
            onChange={onChangeEmail}
            autoCompleteType="email"
            value={email}
          />

          <TextInput
            id="password"
            isPassword
            labelText="Enter your password"
            onChange={onChangePassword}
            value={password}
          />
        </FormContainer>
      </section>
    </>
  )
}

export default LoginPage
