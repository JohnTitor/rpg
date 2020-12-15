# RPG - CLI tool for the Rust Playground

CLI tool for the [Rust Playground](https://play.rust-lang.org).

This is for the **R**ust **P**lay**G**round, so let's call it "RPG".

## Installation

From crates.io:

```sh
# executable as `rpg`
cargo install rpg-cli
```

From `main` branch:
```
cargo install --git https://github.com/JohnTitor/rpg --branch main
```

## Usage

See also `rpg --help`.

### Run

Run your code on the Rust Playground:

```sh
rpg run <file-name>
```

Or, open your default browser with given code:

```sh
rpg run --open <file-name>
```

### Share

Generate permanent playground URL with given code:

```sh
rpg share <file-name>
```

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
