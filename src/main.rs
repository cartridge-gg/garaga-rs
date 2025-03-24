use clap::Parser;
use ed25519_test_generator::generators::signature_generator::generate_ecdsa_test;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(
    version, 
    about = "ED25519 ECDSA test generator for Cairo",
    long_about = None
)]
struct Cli {
    /// Random seed for reproducible test generation
    #[arg(short, long, default_value_t = 0)]
    seed: u64,
    
    /// Output directory
    #[arg(short, long, default_value = "tests")]
    out_dir: String,
}

fn main() {
    let cli = Cli::parse();
    let test_code = generate_ecdsa_test(cli.seed);
    write_test_file(&cli.out_dir, "ecdsa_ED25519_test.cairo", &test_code);
}

fn write_test_file(dir: &str, filename: &str, content: &str) {
    let path = Path::new(dir);
    if !path.exists() {
        fs::create_dir_all(path).expect("Failed to create directory");
    }
    
    let file_path = path.join(filename);
    fs::write(&file_path, content).expect("Failed to write test file");
    println!("Test file written to {}", file_path.display());
} 