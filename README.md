<div align="center">

<samp>

# lfh

A small utility to launch/focus/hide applications for macOS.<br>

:warning: This program is intended for my personal use.

</samp>

</div>

<br>

## Usage

```sh
$ lfh <bundle-id>
```

This command is intended to be used with
[skhd](https://github.com/koekeishiya/skhd), a hotkey daemon for macOS.

My `skhd` configuration looks like this:

```
ctrl + alt - j : lfh org.alacritty
ctrl + alt - k : lfh org.mozilla.firefoxdeveloperedition
# [...]
```

## Installation

Using `cargo`:

```sh
$ cargo install --git https://github.com/ryuma017/lfh
```
