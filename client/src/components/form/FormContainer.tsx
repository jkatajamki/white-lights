export interface FormContainerProps {
  children: React.ReactNode
}

const FormContainer = ({ children }: FormContainerProps) => (
  <>
    <form>
      <div className="form-container-wrapper">
        <fieldset className="form-container">{children}</fieldset>
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
    </form>
  </>
)

export default FormContainer
