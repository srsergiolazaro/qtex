import './style.css'
import { CONFIG } from './config'

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
  if (commandEl) {
    commandEl.textContent = CONFIG.commands[os] || CONFIG.commands.unix
    // Add a small flash effect when switching
    commandEl.style.color = 'white'
    setTimeout(() => {
      commandEl.style.color = 'var(--secondary)'
    }, 200)
  }
}

// Init
function init() {
  // Update static elements from config
  const statusEl = document.querySelector('.status-indicator')
  if (statusEl) {
    const pulse = statusEl.querySelector('.pulse')
    statusEl.innerHTML = ''
    if (pulse) statusEl.appendChild(pulse)
    statusEl.appendChild(document.createTextNode(CONFIG.status))
  }

  const hero = document.querySelector('.hero')
  if (hero) {
    hero.style.setProperty('--version-content', `"${CONFIG.version}"`)
  }

  switchOS(getOS())
}

init()

// Tab clicks
tabs.forEach(tab => {
  tab.addEventListener('click', () => {
    switchOS(tab.dataset.os)
  })
})

// Copy button
copyBtn?.addEventListener('click', async () => {
  const text = commandEl?.textContent || ''
  try {
    await navigator.clipboard.writeText(text)
    const originalText = copyBtn.textContent
    copyBtn.textContent = 'COPIED!'
    copyBtn.style.background = 'var(--secondary)'

    setTimeout(() => {
      copyBtn.textContent = originalText
      copyBtn.style.background = 'var(--accent)'
    }, 2000)
  } catch (e) {
    console.error('Copy failed:', e)
  }
})

// Intersection Observer for stagger animations (fallback for CSS if needed)
const observerOptions = {
  threshold: 0.1
}

const observer = new IntersectionObserver((entries) => {
  entries.forEach(entry => {
    if (entry.isIntersecting) {
      entry.target.classList.add('visible')
    }
  })
}, observerOptions)

document.querySelectorAll('.fade-up').forEach(el => observer.observe(el))
