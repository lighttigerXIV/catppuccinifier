/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
    "./src/pages/Preview/PreviewPage.jsx",
  ],
  theme: {
    extend: {
      colors:{
        "base": "#1e1e2e",
        "crust": "#11111b",
        "text": "#cdd6f4",
        "surface0": "#313244",
        "surface1": "#45475a",
        "mauve": "#cba6f7"
      }
    },
  },
  plugins: [],
}

