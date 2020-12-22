import Head from "next/head"
import LandingView from "../components/landing/Landing"

const IndexPage = (): JSX.Element => (
  <>
    <Head>
      <title>White Lights | Home</title>
    </Head>

    <LandingView />
  </>
)

export default IndexPage
