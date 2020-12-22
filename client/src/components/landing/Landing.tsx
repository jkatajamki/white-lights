import theme from "../../theme/theme"

const LandingView = (): JSX.Element => {
  return (
    <>
      <div className="highlight-background">
        <section className="content flex-content landing">
          <h1>Welcome to White Lights</h1>

          <p>
            White lights is a tool that helps you to x and y. That way you will
            become a better person, yay!
          </p>
        </section>
      </div>

      <section className="links content flex-content landing">
        <p>
          <a href="/login">Have an account already? Sign in from here!</a>
        </p>
        <p>
          <a href="/register">
            Don&apos;t have one yet? No worries - sign up here!
          </a>
        </p>
      </section>

      <style jsx>{`
        .highlight-background {
          background: ${theme.colors.background.secondary};
        }

        .landing {
          margin 3rem auto;
          max-width: 40rem;
        }

        .landing > h1 {
          color: ${theme.colors.text.secondary};
        }

        .links {
          display: flex;
          flex-direction: column;
          max-width: 30rem;
        }

        .links > p {
          margin-block-end: 0;
        }
      `}</style>
    </>
  )
}

export default LandingView
