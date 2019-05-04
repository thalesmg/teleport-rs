# teleport-rs [![Build Status](https://travis-ci.org/thalesmg/teleport-rs.svg?branch=master)](https://travis-ci.org/thalesmg/teleport-rs)

Teleport around your directories.

Inspired by https://bollu.github.io/teleport/

# Install

```bash
make install
```

Make sure your Cargo installation path (typically `$HOME/.cargo/bin`) is in your `$PATH`.

## Release binaries

If you want to use the pre-built binaries, just put them anywhere in your `$PATH` and add the following to your `.zshrc`/`.bashrc` or the like:

```bash
if [ -f "$PATH_YOU_USED/teleport.sh" -a -x "$PATH_YOU_USED/teleport.sh" ]
then
  . $PATH_YOU_USED/teleport.sh
fi
```

Also copy `src/teleport.sh` to `$PATH_YOU_USED` and make it executable (`chmod +x teleport.sh`).

## Shell completion

If you use `zsh`, there is a completion file to use with `tp`. Just copy `src/completions.zsh` to any path in your `$fpath`. You can also add a new path to it. In your `$HOME/.zshrc`:

```bash
export fpath=(YOUR_PATH $fpath)
autoload -U compinit && compinit
```

# Uninstall

```bash
make uninstall
```
