use clap::{Command, IntoApp};
use clap_complete::Shell;
use std::fs::File;
use std::path::Path;

include!("src/cli.rs");

fn generate(s: Shell, app: &mut Command, outdir: &Path, file: &str) {
    let destfile = outdir.join(file);
    println!("dest: {}", destfile.display());
    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
    let mut dest = File::create(destfile).unwrap();

    clap_complete::generate(s, app, "tabell", &mut dest);
}

fn main() {
    let mut app = Options::command();
    app.set_bin_name("tabell");

    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/completions/");

    generate(Shell::Bash, &mut app, &outdir, "bash/tabell");
    generate(Shell::Elvish, &mut app, &outdir, "elvish/tabell");
    generate(Shell::Fish, &mut app, &outdir, "fish/tabell");
    generate(Shell::PowerShell, &mut app, &outdir, "powershell/tabell");
    generate(Shell::Zsh, &mut app, &outdir, "zsh/_tabell");
}