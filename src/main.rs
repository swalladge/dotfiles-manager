extern crate clap;
extern crate sys_info;

use args::Command;
use runner::Runner;

mod app;
mod args;
mod runner;

// exit code structure idea from https://stackoverflow.com/a/30285110
fn main() {
    let exit_code = run();
    std::process::exit(exit_code);
}

fn run() -> i32 {
    let app = app::new();
    let args = args::get_args(app.get_matches());

    let runner = Runner::new(&args);

    let success = match args.command {
        Command::Install => runner.install(),
        // TODO: implement others
        _ => {
            println!("Valid subcommand required!");
            false
        }
    };

    if success {
        return 0;
    } else {
        return 1;
    }

}
