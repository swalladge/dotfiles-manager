# dotfiles-manager

Copyright © 2017 Samuel Walladge

Experimental dotfiles manager in rust. (alpha quality)

[![Build Status](https://travis-ci.org/swalladge/dotfiles-manager.svg?branch=master)](https://travis-ci.org/swalladge/dotfiles-manager)
[![Coverage Status](https://coveralls.io/repos/github/swalladge/dotfiles-manager/badge.svg?branch=master)](https://coveralls.io/github/swalladge/dotfiles-manager?branch=master)

# About

This is a project for PRT455 (Software Engineering Practice).
The idea is to build a dotfiles manager that is fast, stable, and features the
best from a selection of other dotfiles managers and community interest.

Direction for the project taken from initial research, documented in the
[project plan](docs/PRT455-project-plan-SamuelWalladge.pdf), and a
[survey](https://swalladge.id.au/posts/2017/08/07/dotfiles-config-survey.html).

A sub goal of this is to learn rust, so the code may be a bit iffy...

GitHub issues are currently used for tracking progress and documenting
lower level requirements.


# Testing

For continuous integration, all tests are run on every push with Travis.


There are unit tests that can be run through cargo:

```
$ cargo test
```

Integration tests WIP


# Features

- [x] link files from a directory (package) stow-style
- [ ] remove links from a package
- [ ] host specific config
- [ ] run scripts as hooks before and after installing/removing
- [ ] force install/remove links
- [ ] run in test mode (no filesystem changes)
- [ ] integration tests
- [ ] stable cli api


# Usage

```
dotfiles-manager 0.0.1
Samuel Walladge <samuel@swalladge.id.au>
Manages dotfiles

USAGE:
    dotfiles-manager [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -f, --force      Force creating/removing directories and symlinks, overwriting any
                     that exist.
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


# License

     dotfiles-manager - experimental dotfiles manager in rust
     Copyright (C) 2017 Samuel Walladge

     This program is free software: you can redistribute it and/or modify it
     under the terms of the GNU General Public License as published by the Free
     Software Foundation, either version 3 of the License, or (at your option)
     any later version.

     This program is distributed in the hope that it will be useful, but
     WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
     or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
     for more details.

     You should have received a copy of the GNU General Public License along
     with this program.  If not, see <http://www.gnu.org/licenses/>.
