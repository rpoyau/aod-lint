use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::path::PathBuf;

use aod_lint::models::diagnostics::Severity;
use aod_lint::models::rubric::Section8Rubric;
use aod_lint::passes::typst_ast::TypstAstPass;

#[derive(Parser)]
#[command(name = "aod-lint")]
#[command(about = "Axiomatic Fundamentalism & D.E.C. Compiler/Linter for Typst")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Check {
        file: PathBuf,
        #[arg(long, help = "Enforce strict internal AOD mode (no midstream hoisting allowed)")]
        strict: bool,
        #[arg(long, help = "Emit machine-readable JSON for multi-agent ingestion")]
        json: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { file, strict, json } => {
            let path_str = file.to_string_lossy();
            let content = match fs::read_to_string(&file) {
                Ok(c) => c,
                Err(err) => {
                    eprintln!("{} Failed to read {}: {}", "error:".red().bold(), path_str, err);
                    std::process::exit(1);
                }
            };

            let pass = TypstAstPass::new(&path_str, &content, strict);
            let diagnostics = pass.run();

            let mut rubric = Section8Rubric::new();
            rubric.apply_diagnostics(&diagnostics);

            let hoisted_count = diagnostics.iter().filter(|d| d.hoisted).count();

            if json {
                let output = serde_json::json!({
                    "target": path_str,
                    "strict_mode": strict,
                    "converged": rubric.is_converged(),
                    "rubric_bits": rubric.to_bitstring(),
                    "hoisted_dependencies": hoisted_count,
                    "diagnostics": diagnostics,
                });
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
            } else {
                println!("{}", "======================================================================".cyan());
                println!("                  AOD-LINT AXIOMATIC REPORT (Rust Engine)");
                println!("{}", "======================================================================".cyan());
                println!("Target: {}", path_str.bold());
                println!("Mode:   {}", if strict { "STRICT (Internal AOD)" } else { "DIAGNOSTIC (Two-Pass Hoisting)" });
                println!("Hoisted Dependencies: {}", hoisted_count);
                println!(
                    "Section 8 Rubric: [{}] (Converged: {})",
                    rubric.to_bitstring().yellow(),
                    if rubric.is_converged() { "YES".green().bold() } else { "NO".red().bold() }
                );
                println!("{}", "----------------------------------------------------------------------".cyan());

                if diagnostics.is_empty() {
                    println!("{}", "✅ All checks passed. 100% AF compliant.".green().bold());
                } else {
                    for d in &diagnostics {
                        let icon = match d.severity {
                            Severity::Fatal | Severity::Error => "❌".red(),
                            Severity::Warning => "⚠️ ".yellow(),
                            Severity::Info => "💡".blue(),
                        };
                        println!("{} [{:?}] {}", icon, d.severity, d.rule.as_str().bold());
                        println!("   --> {}:{}:{}", d.file, d.line, d.column);
                        println!("   | {}", d.message);
                        if let Some(fix) = &d.suggested_fix {
                            println!("   = suggestion: {}", fix.green());
                        }
                        println!();
                    }
                }
                println!("{}", "======================================================================".cyan());
            }

            let has_fatal = diagnostics.iter().any(|d| d.severity == Severity::Fatal || d.severity == Severity::Error);
            if has_fatal {
                std::process::exit(1);
            }
        }
    }
}
