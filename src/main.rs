extern crate clap;
extern crate sys_info;

use args::Command;
use runner::Runner;

mod app;
mod args;
mod runner;
mod hooks;
mod file_ops;

// exit code structure idea from https://stackoverflow.com/a/30285110
fn main() {
    let exit_code = run();
    std::process::exit(exit_code);
}

fn run() -> i32 {
    let app = app::new();
    let args = match args::get_args(app.get_matches()) {
        Ok(args) => args,
        Err(msg) => {
            println!("Argument error: {}", msg);
            return 1;
        }
    };

    let runner = Runner::new(&args);

    let success = match args.command {
        Command::Install => runner.install(),
        Command::Uninstall => runner.uninstall(),
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
