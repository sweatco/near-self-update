use std::process::Command;

use chrono::{DateTime, Utc};

fn main() {
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let commit_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", commit_hash);

    let utc_datetime: DateTime<Utc> = Utc::now();
    let compilation_datetime = utc_datetime.format("%d/%m/%Y %H:%M:%S%.3f").to_string();
    println!("cargo:rustc-env=COMPILATION_DATETIME={}", compilation_datetime);
}
