extern crate clap;

use args::Command;
use runner::Runner;

mod app;
mod args;
mod runner;

fn main() {
    let app = app::new();
    let args = args::get_args(app);

    let runner = Runner::new(&args);

    let success = match args.command {
        Command::Install => runner.install(),
        // TODO: implement others
        _ => false,
    };

    if success {
        println!("Completed successfully!");
    }

}
