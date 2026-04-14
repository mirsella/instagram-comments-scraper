# instagram-comments-scraper

Rust CLI that logs into Instagram, scrolls through a post's comments, and exports them to CSV.

## How It Works

The scraper opens the Instagram comments view in a browser session driven by `headless_chrome`, logs in if needed, scrolls the comments container until Instagram stops loading more entries, then extracts the comment text with XPath selectors.

After scraping, it prints a few quick stats, including the top first words used in comments, and writes one CSV row per comment.

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

## Notes

- The scraper currently relies on hardcoded selectors, so Instagram UI changes can break it.
- It spoofs a mobile Safari user agent before logging in.
- Setting `HEADLESS` in the environment disables headless mode and shows the browser window.
