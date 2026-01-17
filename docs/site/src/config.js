// Detect current site base URL (e.g., https://srsergiolazaro.github.io/qtex/)
const getBaseUrl = () => {
    const { origin, pathname } = window.location
    // GitHub Pages sites usually have a pathname like /repo-name/
    // We want to ensure we have the trailing slash if it's a directory
    const base = pathname.endsWith('/') ? pathname : pathname.split('/').slice(0, -1).join('/') + '/'
    return `${origin}${base}`.replace(/\/+$/, '') // Remove trailing slash for base
}

const baseUrl = getBaseUrl()

export const CONFIG = {
    baseUrl,
    githubUrl: 'https://github.com/srsergiolazaro/qtex',
    npmUrl: 'https://www.npmjs.com/package/qtex',
    commands: {
        unix: `curl -fsSL ${baseUrl}/install.sh | bash`,
        windows: `irm ${baseUrl}/install.ps1 | iex`
    },
    status: 'SYSTEMS OPERATIONAL',
    version: 'v1.0.0-PRO'
}
