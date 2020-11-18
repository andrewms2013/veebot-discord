[tac-discord-bot]: https://github.com/Veetaha/tac-discord-bot
[rust-toolchain]: https://www.rust-lang.org/tools/install

# veebot-discord

This is a discord bot for me and friends.
It is able to play music from youtube in your voice channel,
search random pony images for you and greet newcommers with custom
welcomming image generated specially for them.

# Installation

As for now, you have to build the bot from sources.

For this you need to have [Rust toolchain installed][rust-toolchain].

There are also some other dependencies to be installed that are specified in
`Dockerfile`, just take a look there.

Then run this to build the bot binary:

```bash
# Compile the program itself
cargo build --release

# The output is the single executable file
./target/release/veebot
```

To build and run the bot in development mode run this:

```bash
cargo run
```

# Configuration

The bot is configured via the environment variables.
The docs for them are not ready yet...

# WIP

Not all of the mentioned features are implemented.
The current status of the project is rewriting the [previous version of it][tac-discord-bot]
to Rust (yeah Rust :D).
