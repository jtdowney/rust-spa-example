/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["app/**/*.tsx", "templates/**/*.html", "src/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography")],
};
