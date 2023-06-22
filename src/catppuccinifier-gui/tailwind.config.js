/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/**/.{vue,js,ts,jsx,tsx}",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
    "./src/*.{vue,js,ts,jsx,tsx}",
  ],
  safelist: [
    'Theme-Latte',
    'Theme-Frappe',
    'Theme-Macchiato',
    'Theme-Mocha'
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
  daisyui: {
    themes: [
      {
        "latte-rosewater": {
          "primary": "#dc8a78",
          "base-100": "#eff1f5",
        },
        "latte-flamingo": {
          "primary": "#dd7878",
          "base-100": "#eff1f5",
        },
        "latte-pink": {
          "primary": "#ea76cb",
          "base-100": "#eff1f5",
        },
        "latte-mauve": {
          "primary": "#8839ef",
          "base-100": "#eff1f5",
        },
        "latte-red": {
          "primary": "#d20f39",
          "base-100": "#eff1f5",
        },
        "latte-maroon": {
          "primary": "#e64553",
          "base-100": "#eff1f5",
        },
        "latte-peach": {
          "primary": "#fe640b",
          "base-100": "#eff1f5",
        },
        "latte-yellow": {
          "primary": "#df8e1d",
          "base-100": "#eff1f5",
        },
        "latte-green": {
          "primary": "#40a02b",
          "base-100": "#eff1f5",
        },
        "latte-teal": {
          "primary": "#179299",
          "base-100": "#eff1f5",
        },
        "latte-sky": {
          "primary": "#04a5e5",
          "base-100": "#eff1f5",
        },
        "latte-sapphire": {
          "primary": "#209fb5",
          "base-100": "#eff1f5",
        },
        "latte-blue": {
          "primary": "#1e66f5",
          "base-100": "#eff1f5",
        },
        "latte-lavender": {
          "primary": "#7287fd",
          "base-100": "#eff1f5",
        },
        "frappe-rosewater": {
          "primary": "#f2d5cf",
          "base-100": "#303446",
        },
        "frappe-flamingo": {
          "primary": "#eebebe",
          "base-100": "#303446",
        },
        "frappe-pink": {
          "primary": "#f4b8e4",
          "base-100": "#303446",
        },
        "frappe-mauve": {
          "primary": "#ca9ee6",
          "base-100": "#303446",
        },
        "frappe-red": {
          "primary": "#e78284",
          "base-100": "#303446",
        },
        "frappe-maroon": {
          "primary": "#ea999c",
          "base-100": "#303446",
        },
        "frappe-peach": {
          "primary": "#ef9f76",
          "base-100": "#303446",
        },
        "frappe-yellow": {
          "primary": "#e5c890",
          "base-100": "#303446",
        },
        "frappe-green": {
          "primary": "#a6d189",
          "base-100": "#303446",
        },
        "frappe-teal": {
          "primary": "#81c8be",
          "base-100": "#303446",
        },
        "frappe-sky": {
          "primary": "#99d1db",
          "base-100": "#303446",
        },
        "frappe-sapphire": {
          "primary": "#85c1dc",
          "base-100": "#303446",
        },
        "frappe-blue": {
          "primary": "#8caaee",
          "base-100": "#303446",
        },
        "frappe-lavender": {
          "primary": "#babbf1",
          "base-100": "#303446",
        },
        "macchiato-rosewater": {
          "primary": "#f4dbd6",
          "base-100": "#24273a",
        },
        "macchiato-flamingo": {
          "primary": "#f0c6c6",
          "base-100": "#24273a",
        },
        "macchiato-pink": {
          "primary": "#f5bde6",
          "base-100": "#24273a",
        },
        "macchiato-mauve": {
          "primary": "#c6a0f6",
          "base-100": "#24273a",
        },
        "macchiato-red": {
          "primary": "#ed8796",
          "base-100": "#24273a",
        },
        "macchiato-maroon": {
          "primary": "#ee99a0",
          "base-100": "#24273a",
        },
        "macchiato-peach": {
          "primary": "#f5a97f",
          "base-100": "#24273a",
        },
        "macchiato-yellow": {
          "primary": "#eed49f",
          "base-100": "#24273a",
        },
        "macchiato-green": {
          "primary": "#a6da95",
          "base-100": "#24273a",
        },
        "macchiato-teal": {
          "primary": "#8bd5ca",
          "base-100": "#24273a",
        },
        "macchiato-sky": {
          "primary": "#91d7e3",
          "base-100": "#24273a",
        },
        "macchiato-sapphire": {
          "primary": "#7dc4e4",
          "base-100": "#24273a",
        },
        "macchiato-blue": {
          "primary": "#8aadf4",
          "base-100": "#24273a",
        },
        "macchiato-lavender": {
          "primary": "#b7bdf8",
          "base-100": "#24273a",
        },
        "mocha-rosewater": {
          "primary": "#f5e0dc",
          "base-100": "#1e1e2e",
        },
        "mocha-flamingo": {
          "primary": "#f2cdcd",
          "base-100": "#1e1e2e",
        },
        "mocha-pink": {
          "primary": "#f5c2e7",
          "base-100": "#1e1e2e",
        },
        "mocha-mauve": {
          "primary": "#cba6f7",
          "base-100": "#1e1e2e",
        },
        "mocha-red": {
          "primary": "#f38ba8",
          "base-100": "#1e1e2e",
        },
        "mocha-maroon": {
          "primary": "#eba0ac",
          "base-100": "#1e1e2e",
        },
        "mocha-peach": {
          "primary": "#fab387",
          "base-100": "#1e1e2e",
        },
        "mocha-yellow": {
          "primary": "#f9e2af",
          "base-100": "#1e1e2e",
        },
        "mocha-green": {
          "primary": "#a6e3a1",
          "base-100": "#1e1e2e",
        },
        "mocha-teal": {
          "primary": "#94e2d5",
          "base-100": "#1e1e2e",
        },
        "mocha-sky": {
          "primary": "#89dceb",
          "base-100": "#1e1e2e",
        },
        "mocha-sapphire": {
          "primary": "#74c7ec",
          "base-100": "#1e1e2e",
        },
        "mocha-blue": {
          "primary": "#89b4fa",
          "base-100": "#1e1e2e",
        },
        "mocha-lavender": {
          "primary": "#b4befe",
          "base-100": "#1e1e2e",
        }
      },
      "light",
      "dark",
    ]
  },
  plugins: [
    require("daisyui")
  ]
}