/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../blog/templates/*.hbs"],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}
