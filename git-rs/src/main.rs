extern crate git2;

use git2::Repository;
use git2::RepositoryState;
use std::process::exit;

fn main() {
    let repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(_) => exit(0)
    };

    match repo.state() {
        RepositoryState::Clean => print!("✓"),
        _ => print!("✗")
    };
}
