Fog Horn
---

A light-weight Discord bot that plays a foghorn sound effect when a specified target joins a voice channel. The bot requires that a GUILD_ID and a TARGET_ID be set in the `voice.rs` file before compiling. The bot takes a discord token in the form of an environment variable with the name `DISCORD_TOKEN`.

# Running the bot

The bot can be built using `cargo build --release` then you can run the bot by executing the binary in the `./target/release/fog-horn`.
