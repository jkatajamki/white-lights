import theme from "../../theme/theme"

export interface NavigationLink {
  title: string
  url: URL
  name: string
}

const navigationLinks = [
  {
    title: "White Lights",
    url: "/",
    name: "Home",
  },
  {
    title: "Sign In to White Lights",
    url: "/login",
    name: "Sign In",
  },
  {
    title: "Sign Up to White Lights",
    url: "/register",
    name: "Sign Up",
  },
]

const Navigation = (): JSX.Element => {
  return (
    <>
      <nav id="navigation">
        <a className="skip-to-content-link" href="#main">
          Skip to main content
        </a>
        <ul>
          {navigationLinks.map((link, i) => (
            <li className="nav-li" key={i}>
              <a href={link.url}>{link.name}</a>
            </li>
          ))}
        </ul>
      </nav>

      <style jsx>{`
        .skip-to-content-link {
          position: absolute;
          left: -10000px;
          top: 0;
          width: 1px;
          height: 1px;
          overflow: hidden;
        }

        .skip-to-content-link:focus {
          left: 0;
          width: auto;
          height: auto;
          overflow: auto;
        }

        #navigation > ul {
          display: flex;
          flex-direction: row;
        }

        .nav-li {
          display: flex;
          width: 100%;
          max-width: 10rem;
          margin-top: 1rem;
        }

        .nav-li > a {
          color: ${theme.colors.text.secondary};
        }
      `}</style>
    </>
  )
}

export default Navigation
