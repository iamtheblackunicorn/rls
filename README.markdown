# RLS :gear: :hammer:

***The popular "ls" utility re-written in Rust.*** :gear: :hammer:

![GitHub CI](https://github.com/iamtheblackunicorn/rls/actions/workflows/rust.yml/badge.svg)

## About :books:

I've wanted to learn some Rust for a while now, `rls` is the culmination of my first efforts into this programming language. Enjoy! ;)

## Building :hammer: :pick:

You will need the following tools installed and available:

- Rust
- Git
- Make

To compile `rls`, follow these steps:

- 1.) Get the source code:
```bash
$ git clone https://$YOUR_GITHUB_TOKEN@github.com/iamtheblackunicorn/rls.git
```
- 2.) Change directory:
```bash
$ cd rls
```
- 3.) Build the source code:
```bash
$ make build
```

## Installation :inbox_tray:

Move the executable on the path `rls/target/release/rls` to the directory where you keep your binary executables. If you are on Linux or Mac OSX, you might have to change permissions like this: `chmod a+x rls`.

## Usage :book:

Using `rls` is quite simple:
- List the contents of a specific directory:
```bash
$ rls my_directory
```
- List the contents of the current directory:
```bash
$ rls
```

## Changelog :black_nib:

### Version 1.0

- initial release
- upload to GitHub

## Note :scroll:

- *RLS :gear: :hammer:* by Alexander Abraham :black_heart: a.k.a. *"The Black Unicorn" :unicorn:*
- Licensed under the MIT license.
