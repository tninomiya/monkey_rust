use super::repl;
use io::{stdin, stdout};
pub fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    repl::start(stdin, stdout());
}
