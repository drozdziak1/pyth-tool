mod cli;
mod util;

use clap::Parser;

use cli::{Cli, Action};

use util::ErrBox;

fn main() -> Result<(), ErrBox> {
    let cli = Cli::parse();

    match cli.action {
        Action::PingAll => {
            println!("Pinging all blockchains");
        }
        
    }
    Ok(())
}
