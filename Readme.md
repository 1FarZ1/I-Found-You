
## I Found You

- I found is A Cli made with rust that helps you see the people that you follow and doesnt follow you ,  offering huge performance and speed.

## Installation

- You can install the cli by running the following command in your terminal

```bash
cargo install ifoundyou
```

## Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12.5", features = ["json"] }
dialoguer = "0.9"
```

## How To Use

- Use  Cmd  in windows  or terminal in  Linux
- run the project by typing the following command in your terminal

```bash
cargo run
```

- You will be asked to enter your github personal access token , you can genereate one by going into the developer settings in your github account and generate one

- once you enter your token you will get a list of the people that you follow and they dont follow you back

## License

- This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Author

- [1FarZ1](www.github.com/1FarZ1)
