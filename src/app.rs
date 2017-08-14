use clap::{App, Arg, SubCommand};


pub fn new() -> App<'static, 'static> {
    App::new("Dotfiles manager")
    .version("0.0.1")
    .author("Samuel Walladge <samuel@swalladge.id.au>")
    .about("Manages dotfiles")
    .arg(
        Arg::with_name("dir")
            .short("d")
            .long("dir")
            .value_name("DIR")
            .help("Set dir to DIR (default is current dir)")
            .takes_value(true),
    )
    .arg(
        Arg::with_name("hostname")
            .short("B")
            .long("hostname")
            .value_name("NAME")
            .help("override computer's hostname to NAME")
            .takes_value(true),
    )
    // TODO: config file options
    .arg(
        Arg::with_name("target")
            .short("t")
            .long("target")
            .value_name("DIR")
            .help("Set target to DIR (default is $HOME)")
            .takes_value(true),
    )
    .arg(Arg::with_name("test").long("no").short("n").help(
        "Do not actually make any filesystem changes or run hooks",
    ))
    .arg(Arg::with_name("verbose").long("verbose").short("v").help(
        "Be verbose",
    ))
    .arg(Arg::with_name("force").long("force").short("f").help(
        "Force creating/removing directories and symlinks, overwriting any that exist.",
    ))
    .subcommand(
        SubCommand::with_name("install")
            .about("install tags/packages")
            .arg(
                Arg::with_name("PACKAGE")
                    .help("package name(s)")
                    .required(true)
                    .multiple(true),
            ),
    )
    .subcommand(
        SubCommand::with_name("remove")
            .about("remove tags/packages")
            .arg(
                Arg::with_name("PACKAGE")
                    .help("package name(s)")
                    .required(true)
                    .multiple(true),
            )
            .alias("uninstall"),
    )
    .subcommand(
        SubCommand::with_name("add")
            .about("add a file to package")
            .arg(
                Arg::with_name("file")
                    .help("dotfile to add/adopt")
                    .required(true),
            )
            .arg(Arg::with_name("host").short("b").long("host").help(
                "add as host-specific",
            ))
            .arg(Arg::with_name("package").short("p").long("package").help(
                "package name to install to",
            )),
    )
}
