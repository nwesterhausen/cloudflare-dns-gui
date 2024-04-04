/** @type {import('tailwindcss').Config} */
export default {
  theme: {
    extend: {},
  },
  content: ['./src/**/*.{js,ts,jsx,tsx}', './index.html'],
  plugins: [require('daisyui')],
}
