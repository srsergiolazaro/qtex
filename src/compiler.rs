use std::path::{Path, PathBuf};
use std::fs;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;
use crate::ui::Spinner;
use colored::*;

const API_BASE: &str = "https://latex.taptapp.xyz";

#[derive(Deserialize)]
pub struct ValidationResponse {
    pub valid: bool,
    pub errors: Option<Vec<ValidationError>>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct ValidationError {
    pub line: Option<u32>,
    pub message: String,
}

pub struct CompileOptions {
    pub output: String,
    pub server: Option<String>,
    pub json: bool,
    pub verify: bool,
    pub watch: bool,
    pub explicit_files: Vec<PathBuf>,
}

pub async fn compile(dir: &Path, options: &CompileOptions) -> Result<(), Box<dyn std::error::Error>> {
    let output_file_name = &options.output;
    let server_url = options.server.as_deref().unwrap_or(API_BASE);
    let silent = options.json;

    let spinner = if silent {
        None
    } else {
        Some(Spinner::new("Preparing compilation..."))
    };

    let absolute_dir = fs::canonicalize(dir)?;
    let mut all_files = Vec::new();
    
    // If explicit files are provided, use only those; otherwise scan directory
    if options.explicit_files.is_empty() {
        find_files(&absolute_dir, &absolute_dir, &mut all_files)?;
    } else {
        for file_path in &options.explicit_files {
            if file_path.exists() && file_path.is_file() {
                let abs_path = fs::canonicalize(file_path)?;
                let rel = abs_path.strip_prefix(&absolute_dir)
                    .map(|p| p.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| file_path.file_name().unwrap_or_default().to_string_lossy().to_string());
                all_files.push(FileObj { path: abs_path, relative: rel });
            }
        }
    }

    // Sort files to prioritize .tex files
    all_files.sort_by(|a, b| {
        let ext_a = a.path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let ext_b = b.path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if ext_a == "tex" && ext_b != "tex" {
            return std::cmp::Ordering::Less;
        }
        if ext_a != "tex" && ext_b == "tex" {
            return std::cmp::Ordering::Greater;
        }
        a.relative.cmp(&b.relative)
    });

    let mut form = Form::new();
    let mut validate_form = Form::new();
    let mut has_latex = false;
    let mut sent_files_count = 0;

    let allowed_exts = [
        "tex", "bib", "sty", "cls", "bst",
        "pdf", "png", "jpg", "jpeg", "eps",
        "csv", "dat", "tsv", "txt",
        "tikz", "otf", "ttf"
    ];

    for file_obj in &all_files {
        let ext = file_obj.path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
        
        if allowed_exts.contains(&ext.as_str()) {
            if file_obj.path.file_name().and_then(|s| s.to_str()) == Some(output_file_name) {
                continue;
            }

            let content = fs::read(&file_obj.path)?;
            
            if ext == "tex" {
                let part = Part::bytes(content.clone()).file_name(file_obj.relative.clone());
                validate_form = validate_form.part("files", part);
                has_latex = true;
            }

            let part = Part::bytes(content).file_name(file_obj.relative.clone());
            form = form.part("files", part);
            sent_files_count += 1;
        }
    }

    if !has_latex {
        if let Some(s) = spinner {
            s.fail("No LaTeX files found.");
        }
        return Ok(());
    }

    if let Some(ref s) = spinner {
        s.update("Validating LaTeX...");
    }

    let client = reqwest::Client::new();

    // Validation
    match client.post(format!("{}/validate", server_url)).multipart(validate_form).send().await {
        Ok(res) => {
            if let Ok(validation) = res.json::<ValidationResponse>().await {
                if !validation.valid {
                    if let Some(s) = spinner {
                        s.stop();
                    }
                    let errors = validation.errors.unwrap_or_default();
                    let details: Vec<String> = errors.iter().map(|e| format!("[Line {:?}] {}", e.line, e.message)).collect();
                    return Err(format!("Validation failed:\n{}", details.join("\n")).into());
                }

                if let Some(warnings) = validation.warnings {
                    if !warnings.is_empty() && !silent {
                        if let Some(ref s) = spinner { s.stop(); }
                        println!("{}", "Validation warnings:".yellow());
                        for warn in warnings {
                            println!("  {} {}", "⚡".yellow(), warn);
                        }
                        // Reinstate spinner or something? Original JS restarts it.
                    }
                }
            }
        }
        Err(_) => {} // Proceed if validation service is down
    }

    if options.verify {
        if let Some(s) = spinner {
            s.succeed("Validation complete!");
        }
        return Ok(());
    }

    if let Some(ref s) = spinner {
        s.update("Compiling via Tachyon-Tex API...");
    }

    let response = client.post(format!("{}/compile", server_url)).multipart(form).send().await?;

    if response.status().is_success() {
        let compile_time = response.headers().get("x-compile-time-ms")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();
        let files_received = response.headers().get("x-files-received")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| sent_files_count.to_string());

        let bytes = response.bytes().await?;
        let output_path = dir.join(output_file_name);
        fs::write(&output_path, bytes)?;

        if let Some(s) = spinner {
            s.succeed(&format!("PDF generated in {}ms", compile_time.bold()));
            if !silent {
                println!("  {} Files: {} processed", "⚡".dimmed(), files_received);
                println!("  {} Path: {}", "📍".dimmed(), output_path.display());
            }
        }

        if options.watch {
            crate::server::notify_clients(&output_path).await;
        }
    } else {
        let status = response.status();
        let err_msg = response.text().await?;
        if let Some(s) = spinner {
            s.fail(&format!("Compilation failed (Status {})", status));
        }
        return Err(format!("Compilation error:\n{}", err_msg).into());
    }

    Ok(())
}

struct FileObj {
    path: PathBuf,
    relative: String,
}

fn find_files(dir: &Path, base_dir: &Path, files: &mut Vec<FileObj>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if name != "node_modules" && !name.starts_with('.') {
                    find_files(&path, base_dir, files)?;
                }
            } else {
                let rel = path.strip_prefix(base_dir).unwrap().to_string_lossy().replace('\\', "/");
                files.push(FileObj { path, relative: rel });
            }
        }
    }
    Ok(())
}
