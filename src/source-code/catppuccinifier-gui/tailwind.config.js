/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
    "./src/pages/Preview/PreviewPage.jsx",
  ],
  theme: {
    extend: {
      colors: { 
        skin: {
          base: "var(--base)",
          mantle: "var(--mantle)",
          crust: "var(--crust)",
          text: "var(--text)",
          surface0: "var(--surface0)",
          surface1: "var(--surface1)",
          rosewater: "var(--rosewater)",
          flamingo: "var(--flamingo)",
          pink: "var(--pink)",
          mauve: "var(--mauve)",
          red: "var(--red)",
          maroon: "var(--maroon)",
          peach: "var(--peach)",
          yellow: "var(--yellow)",
          green: "var(--green)",
          teal: "var(--teal)",
          sky: "var(--sky)",
          sapphire: "var(--sapphire)",
          blue: "var(--blue)",
          lavender: "var(--lavender)",

        }
      }
    },
  },
  plugins: [],
}

