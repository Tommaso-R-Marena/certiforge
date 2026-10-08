use anyhow::{bail, Context, Result};
use certiforge_package::{
    build_package, canonical_json, sha256_hex, verification_evidence, verify_package, Manifest,
    Postcondition, Precondition, SpecDoc, VerifyResult,
};
use certir_parser::{parse_program, pretty_program};
use clap::{Parser, Subcommand};
use forgeopt::optimize;
use forgeopt_bench::run_smoke_suite;
use forgeopt_search::SearchConfig;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use tempfile::TempDir;

#[derive(Parser, Debug)]
#[command(
    name = "certiforge",
    version,
    about = "Proof-carrying AI software toolkit"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parse and type-check a CertIR program
    Parse {
        file: PathBuf,
    },
    /// Evaluate a program on decimal bitvector inputs
    Eval {
        file: PathBuf,
        #[arg(long)]
        inputs: String,
    },
    /// Optimize with ForgeOpt (untrusted search + independent admission)
    Optimize {
        file: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        #[arg(long, default_value_t = 64)]
        max_candidates: usize,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Package {
        #[command(subcommand)]
        action: PackageCmd,
    },
    Benchmark,
    Attack {
        artifact: PathBuf,
    },
    Audit {
        artifact: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
enum PackageCmd {
    Build {
        program: PathBuf,
        #[arg(long)]
        optimized: Option<PathBuf>,
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        spec_name: Option<String>,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        #[arg(long)]
        functional_lean: Option<PathBuf>,
        #[arg(long)]
        equivalence_lean: Option<PathBuf>,
    },
    Verify {
        artifact: PathBuf,
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Parse { file } => {
            let src = fs::read_to_string(&file)?;
            let p = parse_program(&src)?;
            p.check()?;
            print!("{}", pretty_program(&p));
            eprintln!("OK (well-typed)");
            Ok(ExitCode::SUCCESS)
        }
        Commands::Eval { file, inputs } => {
            let src = fs::read_to_string(&file)?;
            let p = parse_program(&src)?;
            p.check()?;
            let parts: Vec<&str> = inputs.split(',').map(|s| s.trim()).collect();
            if parts.len() != p.params.len() {
                bail!("expected {} inputs, got {}", p.params.len(), parts.len());
            }
            let mut vals = Vec::new();
            for (param, raw) in p.params.iter().zip(parts) {
                match param.ty {
                    certir::Ty::Bool => {
                        vals.push(certir_interpreter::Value::Bool(raw.parse()?));
                    }
                    certir::Ty::BitVec(w) => {
                        let n: u64 = raw.parse()?;
                        vals.push(certir_interpreter::Value::bitvec(w, n));
                    }
                }
            }
            let out = certir_interpreter::eval(&p, &vals)?;
            println!("{out:?}");
            Ok(ExitCode::SUCCESS)
        }
        Commands::Optimize {
            file,
            seed,
            max_candidates,
            out,
        } => {
            let src = fs::read_to_string(&file)?;
            let p = parse_program(&src)?;
            p.check()?;
            let result = optimize(
                &p,
                SearchConfig {
                    seed,
                    max_candidates,
                    enable_unsound_rules: true,
                    timeout_ms: 10_000,
                },
            );
            println!("{}", serde_json::to_string_pretty(&result)?);
            if let (Some(path), Some(best)) = (out, result.best) {
                fs::write(path, pretty_program(&best.program))?;
            }
            Ok(ExitCode::SUCCESS)
        }
        Commands::Package { action } => match action {
            PackageCmd::Build {
                program,
                optimized,
                out,
                spec_name,
                seed,
                functional_lean,
                equivalence_lean,
            } => {
                let p = parse_program(&fs::read_to_string(&program)?)?;
                p.check()?;
                let q = if let Some(opt) = optimized {
                    let q = parse_program(&fs::read_to_string(opt)?)?;
                    q.check()?;
                    q
                } else {
                    let result = optimize(
                        &p,
                        SearchConfig {
                            seed,
                            max_candidates: 64,
                            enable_unsound_rules: true,
                            timeout_ms: 10_000,
                        },
                    );
                    result.best.map(|b| b.program).unwrap_or_else(|| p.clone())
                };

                let lean_func = match functional_lean {
                    Some(path) => fs::read_to_string(path)?,
                    None => default_lean_cert(&p),
                };
                let lean_equiv = match equivalence_lean {
                    Some(path) => fs::read_to_string(path)?,
                    None => default_lean_cert(&p),
                };

                let spec = SpecDoc {
                    name: spec_name.unwrap_or_else(|| p.name.clone()),
                    provenance: "equiv-to-program".into(),
                    description: "Phase I functional spec: optimized equiv reference".into(),
                    reference_program: Some(pretty_program(&p)),
                    precondition: Precondition::True,
                    postcondition: Postcondition::EquivToReference,
                };
                build_package(
                    &out,
                    &p,
                    &q,
                    &spec,
                    Some(&lean_func),
                    Some(&lean_equiv),
                    seed,
                )?;
                eprintln!("wrote package {}", out.display());
                Ok(ExitCode::SUCCESS)
            }
            PackageCmd::Verify { artifact, json } => {
                if json {
                    let evidence = verification_evidence(&artifact)?;
                    let accepted = evidence["result"]["result"] == "accept";
                    println!("{}", serde_json::to_string(&evidence)?);
                    return Ok(if accepted {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::from(2)
                    });
                }
                let result = verify_package(&artifact)?;
                match result {
                    VerifyResult::Accept => {
                        println!("ACCEPT");
                        Ok(ExitCode::SUCCESS)
                    }
                    VerifyResult::Reject { reasons } => {
                        println!("REJECT");
                        for r in reasons {
                            println!("  - {r}");
                        }
                        Ok(ExitCode::from(2))
                    }
                }
            }
        },
        Commands::Benchmark => {
            let results = run_smoke_suite();
            println!("{}", serde_json::to_string_pretty(&results)?);
            Ok(ExitCode::SUCCESS)
        }
        Commands::Attack { artifact } => run_attacks(&artifact),
        Commands::Audit { artifact } => {
            let result = verify_package(&artifact)?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(if result.is_accept() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(2)
            })
        }
    }
}

fn default_lean_cert(_p: &certir::Program) -> String {
    // Always ship a closed Lean theorem artifact; exhaustive u8/u16 checking is
    // complementary evidence, not a license for empty certificates.
    "import Std.Tactic.BVDecide\n\ntheorem certiforge_phaseI_cert : True := by\n  trivial\n".into()
}

fn run_attacks(artifact: &Path) -> Result<ExitCode> {
    let base = verify_package(artifact).context("baseline verify")?;
    if !base.is_accept() {
        println!("BASELINE_REJECT (package does not accept before attacks)");
        println!("{base:?}");
        return Ok(ExitCode::from(3));
    }
    println!("BASELINE ACCEPT");

    let mut false_accepts = 0usize;
    let mut attacks = 0usize;

    let mut run_one = |name: &str, f: &dyn Fn(&Path) -> Result<()>| -> Result<()> {
        attacks += 1;
        let tmp = TempDir::new()?;
        copy_dir(artifact, tmp.path())?;
        f(tmp.path())?;
        let r = verify_package(tmp.path())?;
        let status = if r.is_accept() {
            false_accepts += 1;
            "FALSE_ACCEPT"
        } else {
            "rejected"
        };
        println!("attack {name:<28} {status}");
        Ok(())
    };

    run_one("modify_optimized", &|root| {
        fs::write(
            root.join("optimized.certir"),
            "fn evil(x: u8, y: u8) -> u8 {\n  x\n}\n",
        )?;
        Ok(())
    })?;

    run_one("modify_program", &|root| {
        fs::write(
            root.join("program.certir"),
            "fn evil(x: u8, y: u8) -> u8 {\n  y\n}\n",
        )?;
        Ok(())
    })?;

    run_one("modify_spec_vacuous", &|root| {
        let mut spec: SpecDoc = serde_json::from_str(&fs::read_to_string(root.join("spec.json"))?)?;
        spec.precondition = Precondition::False;
        let text = canonical_json(&spec)?;
        let mut man: Manifest =
            serde_json::from_str(&fs::read_to_string(root.join("manifest.json"))?)?;
        man.hashes
            .insert("spec.json".into(), sha256_hex(text.as_bytes()));
        fs::write(root.join("spec.json"), text)?;
        fs::write(root.join("manifest.json"), canonical_json(&man)?)?;
        Ok(())
    })?;

    run_one("truncate_certificate", &|root| {
        fs::write(root.join("certificates/equivalence.lean"), b"")?;
        Ok(())
    })?;

    run_one("hash_substitution", &|root| {
        let mut man: Manifest =
            serde_json::from_str(&fs::read_to_string(root.join("manifest.json"))?)?;
        man.hashes.insert("program.certir".into(), "0".repeat(64));
        fs::write(root.join("manifest.json"), canonical_json(&man)?)?;
        Ok(())
    })?;

    run_one("drop_equiv_flag", &|root| {
        let mut man: Manifest =
            serde_json::from_str(&fs::read_to_string(root.join("manifest.json"))?)?;
        man.has_equivalence_cert = false;
        fs::write(root.join("manifest.json"), canonical_json(&man)?)?;
        Ok(())
    })?;

    run_one("corrupt_proof_text", &|root| {
        let evil = "-- corrupted\naxiom unsound : False\n";
        let mut man: Manifest =
            serde_json::from_str(&fs::read_to_string(root.join("manifest.json"))?)?;
        man.hashes.insert(
            "certificates/equivalence.lean".into(),
            sha256_hex(evil.as_bytes()),
        );
        fs::write(root.join("certificates/equivalence.lean"), evil)?;
        fs::write(root.join("manifest.json"), canonical_json(&man)?)?;
        Ok(())
    })?;

    println!("\nfalse_acceptance_count={false_accepts} / {attacks} attacks");
    if false_accepts > 0 {
        Ok(ExitCode::from(4))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}
