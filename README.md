# alert-after

Get a desktop notification after a command finishes executing. Helpful for notifying when long running CLI tasks are completed.

Note: Only works on macOS and Linux right now. I'm open to adding [Windows](https://github.com/frewsxcv/alert-after/issues/2) support, so if you're interested, feel free to open a pull request.

![](http://i.imgur.com/XCTUJfT.gif)

# Install

1. [Install Rust](https://rustup.rs/)
2. `cargo install alert-after`

# Usage

```
aa <command name and args>
```

Get a desktop notification after sleeping for five seconds:

```
aa sleep 5
```

Get a desktop notification after retrieving Google:

```
aa wget google.com
```
