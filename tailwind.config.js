/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
  ],
  theme: {
    extend: {
      fontFamily: {
        'mono': ['JetBrains Mono', 'Fira Code', 'Consolas', 'Monaco', 'monospace'],
      },
      colors: {
        // Catppuccin Macchiato palette
        'catppuccin': {
          'rosewater': '#f4dbd6',
          'flamingo': '#f0c6c6',
          'pink': '#f5bde6',
          'mauve': '#c6a0f6',
          'red': '#ed8796',
          'maroon': '#ee99a0',
          'peach': '#f5a97f',
          'yellow': '#eed49f',
          'green': '#a6da95',
          'teal': '#8bd5ca',
          'sky': '#91d7e3',
          'sapphire': '#7dc4e4',
          'blue': '#8aadf4',
          'lavender': '#b7bdf8',
          'text': '#cad3f5',
          'subtext1': '#b8c0e0',
          'subtext0': '#a5adcb',
          'overlay2': '#939ab7',
          'overlay1': '#8087a2',
          'overlay0': '#6e738d',
          'surface2': '#5b6078',
          'surface1': '#494d64',
          'surface0': '#363a4f',
          'base': '#24273a',
          'mantle': '#1e2030',
          'crust': '#181926',
        },
        // Keep terminal colors for loading screen
        'terminal': {
          'bg': '#181926',
          'text': '#a6da95',
          'cursor': '#cad3f5',
          'dim': '#6e738d',
        }
      },
      animation: {
        'blink': 'blink 1s infinite',
        'type': 'type 3s steps(40, end)',
        'fade-in': 'fadeIn 0.5s ease-in',
      },
      keyframes: {
        blink: {
          '0%, 50%': { opacity: '1' },
          '51%, 100%': { opacity: '0' },
        },
        type: {
          '0%': { width: '0' },
          '100%': { width: '100%' },
        },
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        }
      }
    },
  },
  plugins: [],
}