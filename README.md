# dotfiles-manager

Copyright © 2017 Samuel Walladge

dotfiles manager project for PRT455


WIP

```
Dotfiles manager 0.0.1
Samuel Walladge <samuel@swalladge.id.au>
Manages dotfiles

USAGE:
    dotfiles-manager [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -n, --no         Do not actually make any filesystem changes or run hooks
    -V, --version    Prints version information
    -v, --verbose    Be verbose

OPTIONS:
    -d, --dir <DIR>          Set dir to DIR (default is current dir)
    -B, --hostname <NAME>    override computer's hostname to NAME
    -t, --target <DIR>       Set target to DIR (default is $HOME)

SUBCOMMANDS:
    add        add a file to package
    help       Prints this message or the help of the given subcommand(s)
    install    install tags/packages
    remove     remove tags/packages
```
