mod ui;
mod compiler;
mod server;
mod installer;

use clap::Parser;
use std::path::PathBuf;
use std::process;
use colored::*;
use notify::{Watcher, RecursiveMode, Config};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Parser, Debug)]
#[command(name = "qtex", version, about = "Ultra-fast LaTeX compilation powered by Tachyon-Tex", long_about = None)]
struct Args {
    /// Directory or file(s) to compile. Can specify multiple files.
    paths: Vec<PathBuf>,

    /// Watch for changes and recompile
    #[arg(short, long)]
    watch: bool,

    /// Define output filename (default: output.pdf)
    #[arg(short, long, default_value = "output.pdf")]
    output: String,

    /// Specify Tachyon-Tex server URL
    #[arg(short, long)]
    server: Option<String>,

    /// Update to the latest version
    #[arg(short, long)]
    update: bool,

    /// Only verify LaTeX without compiling
    #[arg(long)]
    verify: bool,

    /// Output result in JSON format
    #[arg(long)]
    json: bool,

    /// Don't open PDF/browser after compilation
    #[arg(long)]
    no_open: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let version = env!("CARGO_PKG_VERSION");

    if args.update {
        if let Err(e) = installer::self_update().await {
            ui::error(&format!("Update failed: {}", e));
            process::exit(1);
        }
        process::exit(0);
    }

    if !args.json {
        println!("{}", format!("\n🌀 qtex CLI v{} (Rust Edition)", version).magenta().bold());
    }

    // Determine directory and explicit files from paths
    let (directory, explicit_files) = if args.paths.is_empty() {
        (PathBuf::from("."), Vec::new())
    } else if args.paths.len() == 1 && args.paths[0].is_dir() {
        // Single directory
        (args.paths[0].clone(), Vec::new())
    } else {
        // One or more files specified
        let files: Vec<PathBuf> = args.paths.iter().filter(|p| p.is_file()).cloned().collect();
        let dir = if let Some(first_file) = files.first() {
            first_file.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from("."))
        } else {
            PathBuf::from(".")
        };
        (dir, files)
    };
    
    // Setup installation and cleanup
    if let Err(e) = installer::setup_installation(version) {
         ui::warn(&format!("Failed to setup installation dir: {}", e));
    }
    if let Err(e) = installer::cleanup_old_versions(version) {
         ui::warn(&format!("Failed to cleanup old versions: {}", e));
    }

    let compile_options = compiler::CompileOptions {
        output: args.output.clone(),
        server: args.server.clone(),
        json: args.json,
        verify: args.verify,
        watch: args.watch,
        explicit_files: explicit_files.clone(),
    };

    if args.watch {
        let port = 4343;
        let output_path = directory.join(&args.output);
        // Watch mode always uses JSON output
        let json_mode = true;
        
        match server::start_server(port, output_path.clone()).await {
            Ok(actual_port) => {
                let view_url = format!("http://localhost:{}/view", actual_port);
                
                // Show info messages (always in watch mode for user feedback)
                ui::info(&format!("Watching for changes in: {}", directory.display().to_string().bold()));
                ui::info(&format!("View PDF at: {}", view_url.blue().underline()));

                // Initial compile
                if let Err(e) = compiler::compile(&directory, &compile_options).await {
                    println!("{}", serde_json::json!({ "event": "compile", "success": false, "error": e.to_string() }));
                } else {
                    println!("{}", serde_json::json!({ "event": "compile", "success": true }));
                }

                // Open browser (unless --no-open is specified)
                if !args.no_open {
                    let _ = open::that(view_url);
                }

                // Watcher setup
                let (mut watcher, mut rx) = async_watcher().expect("Failed to create watcher");
                watcher.watch(&directory, RecursiveMode::Recursive).expect("Failed to watch directory");

                let is_compiling = Arc::new(Mutex::new(false));

                while let Some(res) = rx.next().await {
                    match res {
                        Ok(event) => {
                            if event.kind.is_modify() || event.kind.is_create() {
                                let mut compiling = is_compiling.lock().await;
                                if !*compiling {
                                    // Check extensions
                                    let mut should_recompile = false;
                                    let mut changed_file = String::new();
                                    for path in &event.paths {
                                        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                                            let watch_exts = ["tex", "bib", "sty", "cls", "png", "jpg", "jpeg", "pdf"];
                                            if watch_exts.contains(&ext) && path.file_name().and_then(|s| s.to_str()) != Some(&args.output) {
                                                should_recompile = true;
                                                changed_file = path.display().to_string();
                                                break;
                                            }
                                        }
                                    }

                                    if should_recompile {
                                        *compiling = true;
                                        if json_mode {
                                            println!("{}", serde_json::json!({ "event": "change", "file": changed_file }));
                                        }
                                        if let Err(e) = compiler::compile(&directory, &compile_options).await {
                                            if json_mode {
                                                println!("{}", serde_json::json!({ "event": "compile", "success": false, "error": e.to_string() }));
                                            } else {
                                                ui::error(&format!("Watch compile error: {}", e));
                                            }
                                        } else if json_mode {
                                            println!("{}", serde_json::json!({ "event": "compile", "success": true }));
                                        }
                                        *compiling = false;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if json_mode {
                                println!("{}", serde_json::json!({ "event": "error", "message": format!("{:?}", e) }));
                            } else {
                                ui::error(&format!("Watcher error: {:?}", e));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                if json_mode {
                    println!("{}", serde_json::json!({ "event": "error", "message": format!("Failed to start server: {}", e) }));
                } else {
                    ui::error(&format!("Failed to start server: {}", e));
                }
                process::exit(1);
            }
        }
    } else {
        if let Err(e) = compiler::compile(&directory, &compile_options).await {
            if args.json {
                 println!("{}", serde_json::json!({ "success": false, "error": e.to_string() }));
            } else {
                 ui::error(&format!("Compilation failed: {}", e));
            }
            process::exit(1);
        }

        if !args.verify && !args.json && !args.no_open {
            let output_path = directory.join(&args.output);
            let _ = open::that(output_path);
        }

        if args.json {
            println!("{}", serde_json::json!({ "success": true }));
        }
    }
}

fn async_watcher() -> notify::Result<(notify::RecommendedWatcher, futures::channel::mpsc::Receiver<notify::Result<notify::Event>>)> {
    let (tx, rx) = futures::channel::mpsc::channel::<notify::Result<notify::Event>>(1);

    let watcher = notify::RecommendedWatcher::new(
        move |res| {
            let mut tx = tx.clone();
            futures::executor::block_on(async {
                let _ = tx.try_send(res);
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}
