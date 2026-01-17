#!/usr/bin/env node

import { watch } from 'node:fs';
import { resolve, basename, extname } from 'node:path';
import { parseArgs } from 'node:util';
import { colors, ui } from './src/ui.js';
import { compile } from './src/compiler.js';
import { startServer } from './src/server.js';
import { exec } from 'node:child_process';

// --- Entry Point ---
const args = process.argv.slice(2);

const optionsSchema = {
    watch: { type: 'boolean', short: 'w' },
    output: { type: 'string', short: 'o' },
    help: { type: 'boolean', short: 'h' }
};

async function main() {
    try {
        const { values, positionals } = parseArgs({ args, options: optionsSchema, allowPositionals: true });

        if (values.help || positionals.length === 0) {
            console.log(`
${colors.magenta}${colors.bold}🌀 qtex CLI${colors.reset}
Ultra-fast LaTeX compilation powered by Tachyon-Tex

${colors.bold}USAGE:${colors.reset}
  qtex <directory> [options]

${colors.bold}OPTIONS:${colors.reset}
  -w, --watch           Watch for changes and recompile
  -o, --output <file>   Define output filename (default: output.pdf)
  -h, --help            Show this help message
            `);
            process.exit(0);
        }

        const directory = positionals[0];
        console.log(`${colors.magenta}${colors.bold}\n🌀 qtex CLI v1.0.0 (Vanilla)${colors.reset}\n`);

        if (values.watch) {
            startServer();
            const viewUrl = 'http://localhost:4848/view';
            ui.info(`Watching for changes in: ${colors.bold}${directory}${colors.reset}`);
            ui.info(`View PDF at: ${colors.blue}${colors.underline}${viewUrl}${colors.reset}`);

            await compile(directory, values);

            // Auto-open browser
            const openCmd = process.platform === 'darwin' ? 'open' : process.platform === 'win32' ? 'start' : 'xdg-open';
            exec(`${openCmd} ${viewUrl}`);

            let isCompiling = false;
            watch(directory, { recursive: true }, async (event, filename) => {
                if (filename && !filename.startsWith('.') && !isCompiling) {
                    const ext = extname(filename).toLowerCase();
                    const outputFileName = values.output || 'output.pdf';

                    // Watch for LaTeX files and images, but ignore the output PDF to avoid loops
                    const watchExts = ['.tex', '.bib', '.sty', '.cls', '.png', '.jpg', '.jpeg', '.pdf'];
                    const isOutputFile = basename(filename) === basename(outputFileName);

                    if (watchExts.includes(ext) && !isOutputFile) {
                        isCompiling = true;
                        console.log(`\n${colors.blue}🔄 Change detected in ${filename}, recompiling...${colors.reset}`);
                        await compile(directory, values);
                        isCompiling = false;
                    }
                }
            });
        } else {
            await compile(directory, values);
        }

    } catch (e) {
        ui.error(e.message);
        process.exit(1);
    }
}

main();
