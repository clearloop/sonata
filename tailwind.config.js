/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "jit",
  content: ["./blog/templates/*.hbs"],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}
