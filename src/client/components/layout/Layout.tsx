import Navigation from "./Navigation"
import globalStyles from "./styles.global"

const Layout = ({ children }: Record<string, React.ReactNode>): JSX.Element => {
  return (
    <>
      <div className="page-wrapper">
        <header>
          <Navigation />
        </header>

        <main id="main">{children}</main>
        <style jsx global>
          {globalStyles}
        </style>
      </div>
    </>
  )
}

export default Layout
