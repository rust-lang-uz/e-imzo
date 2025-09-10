<p align="center">
    <img src=".github/assets/header.png" alt="Rust Uzbekistan's {E-IMZO}">
</p>

<p align="center">
    <h3 align="center">E-IMZO integration library for rustaceans.</h3>
</p>

<p align="center">
    <a href="https://t.me/rustlanguz"><img align="center" src="https://img.shields.io/badge/chat-grey?style=flat&logo=telegram&logoColor=383636&labelColor=dea584&color=dea584" alt="Telegram Chat"></a>
    <a href="https://github.com/rust-lang-uz/e-imzo/actions/workflows/test.yml"><img align="center" src="https://img.shields.io/github/actions/workflow/status/rust-lang-uz/e-imzo/test.yml?style=flat&logo=github&logoColor=383636&labelColor=dea584&color=dea584" alt="Test CI"></a>
</p>

## About

Upon interacting with e-imzo service installed as a deamon in a Linux system, Xinux Team ran into many issues while trying to obtain information about keys from the service. Not so everything was documented and websocket was unresponsibe. Therefore, it was decided to develop a specific library to enhance e-imzo integration for Rust ecosystem.

## Features

_WIP_

## Development

The project has `shell.nix` which has development environment preconfigured already for you. Just open your
terminal and at the root of this project:

```bash
# Open in bash by default
nix develop

# If you want other shell
nix develop -c $SHELL

# After entering development environment, inside the
# env, you can open your editor, so your editor will
# read all $PATH and environmental variables, also
# your terminal inside your editor will adopt all
# variables, so, you can close terminal.

# Neovim | VSCode | Zed
vim .    # code . # zed .
```

The development environment has whatever you may need already, but feel free to add or remove whatever
inside `shell.nix`.

## Thanks

- [Bahrom](https://github.com/orzklv) - For creating and maintaining this awesome bot (shameless ad).
- [Rust Template](https://github.com/bleur-org/templates) - For helping to initiate bot faster and proceed with development.

## License

This project is licensed under the MIT or Apache-2.0 license - see the [LICENSE-MIT](LICENSE-MIT) or [LICENSE-APACHE](LICENSE-APACHE) file for details.

<p align="center">
    <img src=".github/assets/footer.png" alt="Rust Uzbekistan's {E-IMZO}">
</p>
