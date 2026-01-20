// Raw GitHub URLs for install scripts
const REPO_BASE = 'https://raw.githubusercontent.com/srsergiolazaro/qtex/main'

export const CONFIG = {
    githubUrl: 'https://github.com/srsergiolazaro/qtex',
    releasesUrl: 'https://github.com/srsergiolazaro/qtex/releases/latest',
    commands: {
        unix: `curl -fsSL ${REPO_BASE}/install.sh | bash`,
        windows: `irm ${REPO_BASE}/install.ps1 | iex`
    },
    status: 'SYSTEMS OPERATIONAL',
    version: 'RUST EDITION'
}
