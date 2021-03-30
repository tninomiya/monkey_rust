use repl::repl;
use std::io::{stdin, stdout};
pub fn main() -> std::io::Result<()> {
    let stdin = stdin();
    let stdin = stdin.lock();
    repl::start(stdin, stdout())
}
