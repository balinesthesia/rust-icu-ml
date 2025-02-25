# Rust ICU ML

Hey, We're intensivist & anesthesiologits hacking Rust + ML to make ICU & OT life less insane. Follow or mess—steal it, break it, join in!

## Why Rust?

Fast, safe, perfect for ICU gear. Scary? Nah—start here:

- [Rust by Example](https://github.com/rust-lang/rust-by-example): Quick code to play with.
- [The Rust Book](https://github.com/rust-lang/book): Deep dive if you’re hooked.

## Getting Started

### Status

- v0.1.1: HR (tachycardia + bradycardia) + O2 alerts—Bali ICU in Rust!

### Install Rust (Welcome to Rust!)

Never used Rust? No sweat—5 mins to get started on any machine:

1. **Open your terminal** (Windows: Git Bash, Mac/Linux: Terminal).
2. **Run this command** (copy/paste, hit Enter):

   ```rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Follow the prompts**—hit Enter for defaults, or pick your path (takes 2 mins).
4. **Restart terminal** (close, reopen) or run:
   ```rust
   source $HOME/.cargo/env
   ```
5. **Check Rust**—type:
   ```
   rustc --version
   ```
   Should show something like `rustc 1.80.0` (or newer). Boom, you’re in!

_Tips_:

- Windows users: Might need Visual Studio Build Tools (pops up—install it, 5 mins).
- Stuck? Hit [Rust’s Install Page](https://www.rust-lang.org/tools/install) or ping me!

### Run It

- Clone: `git clone https://github.com/balinesthesia/rust-icu-ml.git`
- Run: `cd pulse_checker && cargo run`
- Play: Swap 120 and 92—see alerts!
- New? Hit [Rust by Example](https://github.com/rust-lang/rust-by-example)—start with “Hello, World” then tweak 120!

### Roadmap

- v0.1.x: ICU basics (HR, O2 alerts).
- v1.0.0: ML-ready—sepsis prediction, smart alarms on Pi 5, Bali ICU power!
- v2.0.0 and beyond!

## For Juniors

No code exp? No stress. Check the links above—10 mins to “Hello, World!” then mess with this. Tag us if it breaks!
