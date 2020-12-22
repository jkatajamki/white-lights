import css from "styled-jsx/css"
import theme from "../../theme/theme"

export default css.global`
  body {
    margin: 0;
    padding: 0;
    box-sizing: border-box;

    background: ${theme.colors.background.primary};
    color: ${theme.colors.text.primary};
    font-family: ${theme.typography.paragraph.fontFamily};
    font-size: 125%;
  }

  a {
    color: ${theme.colors.text.tertiary};
  }

  a:focus {
    outline-color: ${theme.colors.text.secondary};
    outline-width: 0.2rem;
  }

  h1,
  h2,
  h3,
  h4,
  h5 {
    font-family: ${theme.typography.heading.fontFamily};
  }

  h1 {
    font-size: 3rem;
  }

  .content {
    margin: 0 auto;
    max-width: 50rem;
  }

  .flex-content {
    display: flex;
    flex-direction: column;
    padding: 1rem;
  }
`
