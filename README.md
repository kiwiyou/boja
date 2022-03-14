# boja

`boja` (**B**aekjoon **O**nline **J**udge **A**ssistant) is a CLI tool to automate BOJ submitting workflow.

More specifically, `boja` should (but cannot yet):

- [] Find a problem to solve from BOJ

- [x] Create source file(s) from templates

- [x] Test code with given inputs and outputs

- [] Submit code to BOJ platforms

## Usage

`boja` has not published any release, so you should build from source.

Before building, you need `rustc` and `cargo` installed. Latest stable version is recommended.

After that, clone this repository and get into source directory.

```bash
git clone https://github.com/kiwiyou/boja
cd boja
```

Then you can take one of two paths:

1. Run `boja` directly in the source directory

Execute `cargo run` or `cargo run -- <arguments>` if you have arguments to specify.

The build process will begin on your first run.

2. Install `boja` and use the binary

Execute `cargo install .` to build `boja` and install at `.cargo/bin` in your home directory.

If `.cargo/bin` is in your `$PATH` environment variable, type `boja` to run `boja`.
