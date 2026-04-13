# instagram-comments-scraper

Rust CLI that logs into Instagram, scrolls through a post's comments, and exports them to CSV.

## Requirements

- `insta_user`
- `insta_pass`

## Run

```bash
export insta_user="your_username"
export insta_pass="your_password"
cargo run -- "https://www.instagram.com/p/.../"
```

The output file is written as `comments-<post-id>.csv`.

## Build

```bash
cargo build --release
```
