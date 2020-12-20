import theme from "../../theme/theme"

export interface FormContainerProps {
  children: React.ReactNode
}

const FormContainer = ({ children }: FormContainerProps) => (
  <>
    <div className="form-container-wrapper">
      <div className="form-container">{children}</div>
    </div>

    <style jsx>{`
      .form-container-wrapper {
        margin-bottom: 2rem;
      }

      .form-container {
        max-width: 30rem;
        margin: 1rem auto;
      }
    `}</style>
  </>
)

export default FormContainer
