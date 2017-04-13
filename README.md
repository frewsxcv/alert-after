# alert-after

Get a desktop notification after a command finishes executing. Helpful for notifying when long running CLI tasks are completed. Works on macOS, Linux, and Windows.

![](http://i.imgur.com/XCTUJfT.gif)

# Install

1. [Install Rust](https://rustup.rs/)
2. `cargo install alert-after`

If installing on Linux, [Libdbus is also required](https://github.com/diwic/dbus-rs#requirements).

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
