# A Little Rusty Chat

![](https://raw.githubusercontent.com/thedenisnikulin/micro-uwu-rusty-chat/main/mean-capybara-demo.gif)

## Usage
```
USAGE:
    mean-capybara [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -s, --server     Launches the program in server mode.
    -V, --version    Prints version information

OPTIONS:
    -a, --address <address>    Address to bind the server on [default: 127.0.0.1]
    -p, --port <port>          Port to bind the server on

Launch the program without the "server" argument to use it as a client.
```

## Build and run
```
git clone https://github.com/thedenisnikulin/micro-uwu-rusty-chat
cd micro-uwu-rusty-chat/
cargo build --release
./target/release/mean-capybara
```
