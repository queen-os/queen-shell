use queen_shell::{cli, shell::StdShell};
use std::sync::Arc;

fn main() {
    let shell = Arc::new(StdShell::new());
    futures_lite::future::block_on(cli(shell)).unwrap();
}
