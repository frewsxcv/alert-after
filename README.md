# alert-after

Get a desktop notification after a command finishes executing. Helpful for notifying when long running CLI tasks are completed. Works on macOS, Linux, and Windows.

![](http://i.imgur.com/XCTUJfT.gif)

## Install

1. [Install Rust](https://rustup.rs/)
2. `cargo install alert-after`

If installing on Linux, [Libdbus is also required](https://github.com/diwic/dbus-rs#requirements).

## Usage

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

## Upgrade

```
cargo install --force alert-after
```

## Windows >= 8

```shell
$ aa sleep 5
C:/Users/user/.cargo/bin/aa.exe: error while loading shared libraries: api-ms-win-core-winrt-string-l1-1-0.dll: cannot open shared object file: No such file or directory
```

>Note that winrt requires at least Windows 8 (I have only tested in on Windows 10, but the WinRT was introduced in Windows 8, so it should work).
>
>Furthermore, the example in the README aa sleep 5 doesn't work on Windows: In good old cmd.exe there is no sleep command; in Powershell there is a sleep command, but it's a shell built-in (Cmdlet), so it won't work with aa (related to #8). There is an alternative timeout command that does something similar and works in cmd and Powershell, though. The wget example similarly won't usually work, because wget is usually not installed on Windows. Powershell has a default alias wget for the Invoke-WebRequest Cmdlet, but again, this won't work with aa, since there is no actual executable called wget.

from https://github.com/frewsxcv/alert-after/issues/2#issuecomment-293819992
