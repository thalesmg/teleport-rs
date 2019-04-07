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

# Uninstall

```bash
make uninstall
```
