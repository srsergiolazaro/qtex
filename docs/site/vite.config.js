import { defineConfig } from 'vite'

export default defineConfig({
    // Base path for GitHub Pages (repo name)
    base: '/qtex/',

    build: {
        // Output to dist folder
        outDir: 'dist',

        // Minify for production
        minify: 'esbuild',

        // Target modern browsers
        target: 'es2020'
    },

    // Dev server config
    server: {
        port: 5173,
        open: true
    },

    // Preview server config
    preview: {
        port: 4173
    }
})
