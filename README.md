# The Nebula
### A TMT Joint
* Tyler Nicholls
* Michael Spagnolo
* Tim Wilson

Written in Rust.

Try to knock the circle shooting bullets (enemy) into the nebula (purple circle).
The nebula does damage to the enemy. If you get the enemy's health to zero,
you win. If you get hit by three of the enemy's bullets, you lose.


## Installing Rust and running The Nebula on Ubuntu Linux:

To install Rust, in a terminal, type:
`
$ curl https://sh.rustup.rs -sSf | sh
`

Test your install by typing:
`
$ rustc --version
`
You should see a version number and some other information

### Cargo
Cargo is Rust's build manager, similar to make. It also manages packages,
called crates. It should be installed automatically when Rust was installed.
To make sure it is installed, type:
`
$ cargo --version
`
You should see a version number.

To use cargo, a Rust project should be organized such that the source
file, main.rs, is in a directory /src and there needs to be a Cargo
configuration file, Cargo.toml. This project should already be organized
correctly with the appropriate Cargo.toml file.

### building and running
To build (compile) and run the project, in the project's root directory, type:
`
$ cargo run
`

The executable will be in /target/debug/ (named: the_nebula) To build
without running, type:
`
$ cargo build
`

To compile the code to a more optimized, release version of the executable,
you can compile with the --release flag. This takes much longer to compile,
but results in a faster executable that is part of the attraction of
Rust. To do this, type:
`
$ cargo build --release
`

### Helpful Code:
We used this code as a jumping off point. Sure wish there was more explanation, but
it got us started:
[Piston-Tutorials](https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started)
