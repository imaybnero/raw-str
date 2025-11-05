`rawstring` provides a string type that can hold data that may or not be valid UTF-8.

# Overview
Rust's standard [`String`] and [`&str`](str) types require valid UTF-8,
which can be limiting when working with arbitrary binary data.
This crate provides the following types as alternatives:
- [`RawStr`]: a borrowed view of raw string data
- [`RawString`]: an owned raw string type

[`RawStr`] and [`RawString`] differ from `&[u8]` and `Vec<u8>` in that they
are treated as text rather than binary data. For example, both types' [`Debug`] and [`Display`]
implementations handle invalid UTF-8 gracefully by replacing invalid sequences
with the Unicode replacement character (`�`). This makes them suitable for logging,
user output, and other contexts where human-readable text is desired.

# License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.