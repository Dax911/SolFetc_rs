/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs", "./index.html"],
  theme: {
    extend: {
      colors: {
        "cyber-cyan": "#00f0ff",
        "neon-rose": "#ff2d78",
        "vault-dark": "#0a0e17",
        "panel-dark": "#111827",
        "glass-border": "rgba(255,255,255,0.08)",
        "text-muted": "#8892a4",
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
        mono: ["JetBrains Mono", "monospace"],
      },
      boxShadow: {
        glow: "0 0 20px rgba(0, 240, 255, 0.15)",
        "glow-rose": "0 0 20px rgba(255, 45, 120, 0.15)",
      },
      backdropBlur: {
        xl: "24px",
      },
    },
  },
  plugins: [],
};
