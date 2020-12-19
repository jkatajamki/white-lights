import globalStyles from "./styles.global"

const Layout = ({ children }: Record<string, React.ReactNode>): JSX.Element => (
  <div className="page-wrapper">
    {children}
    <style jsx global>
      {globalStyles}
    </style>
  </div>
)

export default Layout
