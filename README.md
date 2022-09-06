# Microbit Text Scroller

This lib provides a simple ScrollMessage struct that would allow you to have scrolling text on your microbit 5*5 LED display

Rationale:

Initially I thought this functionality already been implemented and is nicely available, however I only managed to find [1 crate](https://github.com/mattheww/microbit-text) that should have worked for me,
but it didn't (still not sure why), sooo that's the main reason for me creating this lib (plus I wanted to practice some Rust)

So anyhow it was a great experience for me and maybe this lib would be of use to someone else out there


## Usage
This lib is not available on `crates.io`, so you'd need to add it to your Cargo.toml as following
```
[dependencies]
microbit-text-scroller = { git = "https://github.com/MikhailMS/microbit-text-scroller" }
```

## Examples
While this is super easy to use lib, if you feel like you want to see some examples - [check out this repo](https://github.com/MikhailMS/microbit-discovery-examples)
