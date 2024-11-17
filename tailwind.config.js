/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["*.html", "./src/**/*.rs", "./src/**/*.html", "./style/**/*.css"],
    theme: {
      extend: {
        keyframes: {
          slideDown: {
            '0%': { transform: 'translateY(-100%) translateX(-50%)', opacity: '0' },
            '100%': { transform: 'translateY(0%) translateX(-50%)', opacity: '1' },
          },
          slideDownNoTranslateX: {
            '0%': { transform: 'translateY(-100%)', opacity: '0' },
            '100%': { transform: 'translateY(0%)', opacity: '1' },
          },
          scaleAnimation: {
            '0%': { transform: 'scale(0)', opacity: '0' },
            '100%': { transform: 'scale(1)', opacity: '1' },
          },
          textAnimation: {
            '0%': { transform: 'translateY(100%)', opacity: '0' },
            '100%': { transform: 'translateY(0%)', opacity: '1' },
          },
          slideHorizontal: {
            '0%': { transform: 'translateX(0%)', opacity: '0' },
            '100%': { transform: 'translateX(0%)', opacity: '1' },
          },
        },
        animation: {
          'slide-down': 'slideDown 0.5s ease-out forwards',
          'slide-down-no-translate-x': 'slideDownNoTranslateX 0.5s ease-out forwards',
          'image-animation': 'scaleAnimation 0.2s ease-out forwards',
          'span-animation': 'scaleAnimation 0.7s cubic-bezier(0.83, 0, 0.5, 1.73) forwards',
          'text-animation': 'textAnimation 0.5s',
          'text-animation-fast': 'textAnimation 0.175s',
          'slide-horizontal': 'slideHorizontal 0.2s ease-out forwards',
        },
      },
    },
    plugins: [],
  }