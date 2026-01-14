export const ESC = '\x1b[';
export const colors = {
    reset: `${ESC}0m`,
    bold: `${ESC}1m`,
    dim: `${ESC}2m`,
    magenta: `${ESC}35m`,
    blue: `${ESC}34m`,
    cyan: `${ESC}36m`,
    green: `${ESC}32m`,
    yellow: `${ESC}33m`,
    red: `${ESC}31m`,
};

export const ui = {
    log: (msg) => console.log(msg),
    error: (msg) => console.log(`${colors.red}✖ ${msg}${colors.reset}`),
    warn: (msg) => console.log(`${colors.yellow}⚠️  ${msg}${colors.reset}`),
    success: (msg) => console.log(`${colors.green}✔ ${msg}${colors.reset}`),
    info: (msg) => console.log(`${colors.cyan}ℹ ${msg}${colors.reset}`),
    clearLine: () => process.stdout.write('\r\x1b[K'),
};

export class Spinner {
    constructor(text) {
        this.text = text;
        this.frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        this.index = 0;
        this.timer = null;
    }
    start() {
        this.timer = setInterval(() => {
            process.stdout.write(`\r${colors.cyan}${this.frames[this.index]}${colors.reset} ${this.text}`);
            this.index = (this.index + 1) % this.frames.length;
        }, 80);
        return this;
    }
    update(text) {
        this.text = text;
    }
    stop(symbol = ' ', color = colors.reset) {
        clearInterval(this.timer);
        ui.clearLine();
    }
    succeed(text) {
        this.stop();
        console.log(`${colors.green}✔${colors.reset} ${text}`);
    }
    fail(text) {
        this.stop();
        console.log(`${colors.red}✖${colors.reset} ${text}`);
    }
}
