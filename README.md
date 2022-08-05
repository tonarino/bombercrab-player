# The Bombercrab Challenge (Player Template)

Welcome to the 2022 Tokyo Game Hack bombercrab challenge!

If you're based in Tokyo and planning to attend in person, please let us know in the 
[Peatix event page](https://bombercrab-rust-game-hack.peatix.com/view) so we can get started
on your pizza. If you'll instead be joining us remotely, send us a message using any channel
you prefer so we can hand you an API key, and in either case keep an eye on our
[twitch channel](https://www.twitch.tv/rust_bombercrab) the 12th of August at 7PM JST.

# How to participate

Whether in person or remotely, we will give you an API key so you can participate in the live games. You are encouraged to
upload your character as often as you like; you'd be queued so everyone has a fair chance to play, but don't hesitate
to constantly improve your character's behaviour, even while a game is currently going on.

# One time setup

Install Rust toolchain for the `wasm32-unknown-unknown` target. If using https://rustup.rs/, run `rustup target add wasm32-unknown-unknown`.

The `submit.sh` script also requires having the `curl` binary around.

Copy `.env.example` to `.env` and edit the contents with the server IP address (which we'll provide when the event starts), 
the API key (which we'll also provide) and the name of your player crate (you can leave this as `player` if you like).

# Instructions

1. Edit code of your bomber agent in `player/src/`.
2. Submit your code using `./submit.sh`.
3. Goto (1) and iterate on your logic, your character's brain is 
   reloaded live whenever you run the script!

# Goal of the game

The objective of the game is to spend as much time as possible on a `Hill` tile, and ensure you're alive at the end of each
three minute round. Beyond that, the API should tell you all you need to know about the world surrounding your character.

Teams are cosmetic only; signing up with a specific team name will assign you a color in common with all other players of the 
same team. You can program custom behaviours based on what team your opponents belong to, so feel free to come up with team 
strategies!
