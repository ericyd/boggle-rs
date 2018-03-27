# boggle-rs
Command line version of Boggle®, written in Rust

## Playing

Download binaries from [releases](https://github.com/ericyd/boggle-rs/releases). Windows and Linux binaries are available.

## Building from source
1. [Install Rust](https://www.rust-lang.org/en-US/install.html)
2. clone repo: `git clone https://github.com/ericyd/boggle-rs.git`
3. head on in there! `cd boggle-rs`
4. Build and or play! `cargo build` and/or `cargo run`

Built on Linux and Windows with
* cargo 0.25.0 (2018-02-26)
* rustc 1.24.1 (2018-02-27)

On Linux, use `strip boggle` on the release binary for substantial size savings.
[credit](https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html)



## Expected Output

The game looks something like this in your console

```
Welcome to Boggle®
==================

Hello eric, here is your game:
Enter as many words as possible in 0.3 mins!

 R  I  P  I 

 X  H  M  F 

 I  B  G  E 

 R  L  O  U 



Now start typing words! (18 seconds left)

```
