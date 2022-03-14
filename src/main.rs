mod shell;

use std::{collections::HashMap, fs::File, io::Write, path::PathBuf, process::Stdio};

use clap::{Parser, Subcommand};
use serde::Deserialize;

/// Online Judge Assistant
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a new problem directory
    New {
        /// Name of the problem directory
        name: String,
        /// Language to solve the problem in
        language: String,
        /// Code templates to use
        templates: Vec<String>,
    },
    /// Test a solution file against *.in and *.out files
    Test {
        /// Name of the problem directory
        name: String,
        /// Language to test the solution in
        language: String,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Command::New {
            name,
            language,
            templates,
        } => {
            let config = load_configuration().expect("cannot load configurations");
            let lang = config.lang.get(language).expect("cannot find language");
            let solution_path: PathBuf = ["solution", name].iter().collect();
            std::fs::create_dir_all(&solution_path).expect("cannot create solution directory");
            let source_path = solution_path.join(&lang.source);
            let mut solution = File::create(&source_path).expect("cannot open solution file");
            let template_path = PathBuf::from("templates");
            for template in templates {
                let mut template = template_path.join(&template);
                if let Some(extension) = source_path.extension() {
                    template.set_extension(extension);
                }
                let mut template_file = File::open(template).expect("cannot open template file");
                std::io::copy(&mut template_file, &mut solution)
                    .expect("cannot copy template file");
            }
        }
        Command::Test { name, language } => {
            let config = load_configuration().expect("cannot load configurations");
            let lang = config.lang.get(language).expect("cannot find language");
            let solution_path: PathBuf = ["solution", name].iter().collect();
            if let Some(compile) = &lang.compile {
                let compile_output = shell::make_shell_command(compile)
                    .stderr(Stdio::piped())
                    .stdout(Stdio::piped())
                    .current_dir(&solution_path)
                    .output()
                    .expect("cannot compile solution");
                if !compile_output.status.success() {
                    eprintln!("compilation failed");
                    std::io::stderr().write_all(&compile_output.stderr).unwrap();
                    return;
                }
            }
            for file in solution_path
                .read_dir()
                .expect("cannot read solution directory")
            {
                let file = file.expect("cannot read file");
                let file_name = file.file_name();
                if let Some(utf8_name) = file_name.to_str() {
                    if !utf8_name.ends_with(".in") {
                        continue;
                    }
                } else {
                    continue;
                }
                let input_path = file.path();
                let output_path = input_path.with_extension("out");
                if !output_path.exists() {
                    continue;
                }
                let input_reader =
                    std::fs::File::open(&input_path).expect("cannot open input file");
                let test_output = shell::make_shell_command(&lang.execute)
                    .arg(&input_path)
                    .current_dir(&solution_path)
                    .stdin(input_reader)
                    .output()
                    .expect("cannot execute solution");
                if !test_output.stderr.is_empty() {
                    eprintln!("Runtime Error");
                    std::io::stderr().write_all(&test_output.stderr).unwrap();
                    return;
                }
                let test_output = String::from_utf8(test_output.stdout)
                    .expect("output is not a valid utf8 string");
                let output =
                    std::fs::read_to_string(&output_path).expect("cannot read output file");
                let line_count = test_output.lines().count();
                if test_output.lines().eq(output.lines().take(line_count)) {
                    println!(
                        "Test {} Accepted",
                        input_path.file_stem().unwrap().to_string_lossy()
                    );
                } else {
                    println!(
                        "Test {} Failed",
                        input_path.file_stem().unwrap().to_string_lossy()
                    );
                    break;
                }
            }
        }
    }
}

#[derive(Deserialize)]
struct Config {
    lang: HashMap<String, Language>,
}

#[derive(Deserialize)]
struct Language {
    source: String,
    compile: Option<String>,
    execute: String,
}

fn load_configuration() -> std::io::Result<Config> {
    let config = PathBuf::from("config.toml");
    if !config.exists() {
        std::fs::write(&config, include_str!("default-config.toml"))?;
    }
    let content = std::fs::read(&config)?;
    toml::from_slice(&content).map_err(Into::into)
}
