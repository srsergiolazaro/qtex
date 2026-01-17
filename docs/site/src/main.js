import './style.css'

// OS Detection & Command Switching
const commands = {
  unix: 'curl -fsSL https://srsergiolazaro.github.io/qtex/install.sh | bash',
  windows: 'irm https://srsergiolazaro.github.io/qtex/install.ps1 | iex'
}

const tabs = document.querySelectorAll('.os-tab')
const commandEl = document.getElementById('install-command')
const copyBtn = document.getElementById('copy-btn')

// Detect OS
function getOS() {
  const ua = navigator.userAgent.toLowerCase()
  if (ua.includes('win')) return 'windows'
  return 'unix'
}

// Switch command
function switchOS(os) {
  tabs.forEach(t => t.classList.toggle('active', t.dataset.os === os))
  if (commandEl) commandEl.textContent = commands[os]
}

// Init with detected OS
switchOS(getOS())

// Tab clicks
tabs.forEach(tab => {
  tab.addEventListener('click', () => switchOS(tab.dataset.os))
})

// Copy button
copyBtn?.addEventListener('click', async () => {
  const text = commandEl?.textContent || ''
  try {
    await navigator.clipboard.writeText(text)
    copyBtn.textContent = '[COPIED]'
    setTimeout(() => copyBtn.textContent = '[COPY]', 1500)
  } catch (e) {
    console.error('Copy failed:', e)
  }
})
