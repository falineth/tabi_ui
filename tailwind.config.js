/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    // Include your component source files
    "./src/**/*.rs",
    "./examples/**/*.rs",
  ],
  theme: {
    extend: {
      // Add your custom theme extensions here
    },
  },
  plugins: [
    // Add your Tailwind plugins here
  ],
};
