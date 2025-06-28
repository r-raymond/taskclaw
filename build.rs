use clap::CommandFactory;
use clap_complete::{generate_to, shells::*};
use clap_mangen::Man;
use std::env;
use std::fs;
use std::path::PathBuf;

include!("src/cli.rs");

fn main() -> std::io::Result<()> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = Cli::command();

    // Generate man page
    let man = Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let man_dir = PathBuf::from(&outdir).join("man");
    fs::create_dir_all(&man_dir)?;
    fs::write(man_dir.join("claw.1"), buffer)?;

    // Generate shell completions
    let comp_dir = PathBuf::from(&outdir).join("completions");
    fs::create_dir_all(&comp_dir)?;

    generate_to(Bash, &mut cmd, "claw", &comp_dir)?;
    generate_to(Zsh, &mut cmd, "claw", &comp_dir)?;
    generate_to(Fish, &mut cmd, "claw", &comp_dir)?;
    generate_to(PowerShell, &mut cmd, "claw", &comp_dir)?;

    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=src/main.rs");

    Ok(())
}
