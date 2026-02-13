# Zellij switch session pipe

A small plugin to switch the current zellij session via the cli.

# Installation

To install you need the `wasm32-wasip1` toolchain. You can install it via `rustup` with `rustup target add wasm32-wasip1`

```shell
mkdir -p ~/.local/share/zellij/plugins
git clone https://github.com/tuxikus/zellij-switch-session-pipe.git
cd zellij-switch-session-pipe
cargo build --release
cp ./target/wasm32-wasip1/release/zellij-switch-session-pipe.wasm ~/.local/share/zellij/plugins/
```

# Usage

Bash/ZSH:
```shell
cat <<'EOF' > ~/.local/bin/switch-zellij-session
#!/usr/bin/env bash

# replace tv with fzf, sk or any other fuzzy finder
SESSION=$(zellij list-sessions -s | tv)
zellij pipe --plugin file:"$HOME"/.local/share/zellij/plugins/zellij-switch-session-pipe.wasm -- "$SESSION"
EOF
chmod +x ~/.local/bin/switch-zellij-session
```

Fish:
```fish
begin
  echo '#!/usr/bin/env fish'
  echo ''
  echo '# replace tv with fzf, sk or any other fuzzy finder'
  echo 'set SESSION (zellij list-sessions -s | tv)'
  echo 'zellij pipe --plugin file:"$HOME"/.local/share/zellij/plugins/zellij-switch-session-pipe.wasm -- "$SESSION"'
end > ~/.local/bin/switch-zellij-session

chmod +x ~/.local/bin/switch-zellij-session

```

In zellij config:
```kdl
  bind "Alt s" {
      Run "path/to/script/switch-zellij-session" {}
  }
```
