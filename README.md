# Hex!

GameJam entry for the rad team

# Getting Started

To enable a second window with debug information, enable the 'debugwindow' feature. (`cargo run --features debugwindow`)

# Tooling

## clippy

A collection of lints to catch common mistakes and improve your Rust code.

To see suggestions: `cargo clippy`

To automatically apply suggestions: `cargo clippy --fix`

1. https://github.com/rust-lang/rust-clippy

## rustfmt

A tool for formatting Rust code according to style guidelines.

1. https://github.com/rust-lang/rustfmt
2. https://github.com/rust-lang/rustfmt/blob/master/intellij.md (For use with CLion's Rust Plugin)

# Dependencies

See `cargo.toml` for details.

## bevy

Bevy is a refreshingly simple data-driven game engine built in Rust. It is free and open-source forever!

1. https://bevyengine.org/
2. https://bevyengine.org/learn/
3. https://github.com/bevyengine/bevy
4. https://crates.io/crates/bevy

## bevy-inspector-egui

This crate provides a debug interface using egui where you can visually edit the values of your components live.

1. https://github.com/jakobhellermann/bevy-inspector-egui
2. https://crates.io/crates/bevy-inspector-egui

## bevy_kira_audio

Audio plugin for Bevy.

1. https://github.com/niklasei/bevy_kira_audio
2. https://crates.io/crates/bevy_kira_audio
3. https://bevy-cheatbook.github.io/features/audio.html

## noise

Procedural noise generation library.

1. https://github.com/razaekel/noise-rs
2. https://crates.io/crates/noise
3. https://docs.rs/noise/0.8.2/noise/

## rand

A Rust library for random number generation.

1. https://rust-random.github.io/book/
2. https://docs.rs/rand/latest/rand/
3. https://github.com/rust-random/rand
4. https://crates.io/crates/rand

## ron

Rusty Object Notation (RON) is a simple readable data serialization format that looks similar to Rust syntax.
It's designed to support all of Serde's data model, so structs, enums, tuples, arrays, generic maps, and primitive
values.

1. https://github.com/ron-rs/ron

## serde

Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.

1. https://serde.rs/
2. https://github.com/serde-rs/serde
3. https://crates.io/crates/serde
