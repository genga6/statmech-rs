#!/usr/bin/env bash
set -euo pipefail

log() { echo "[$(date +%H:%M:%S)] $*"; }

# rustfmt / clippy はイメージに入っているはずだが、念のため
install_rust_components() {
  log "rust components: start"
  rustup component add rustfmt clippy >/dev/null 2>&1 || log "rust components: rustup unavailable, skip"
  log "rust components: done"
}

install_typos() {
  log "typos: start"
  if command -v typos >/dev/null 2>&1; then
    log "typos: already installed"
    return
  fi

  local ver url tmp
  ver="$(curl -fsSL https://api.github.com/repos/crate-ci/typos/releases/latest \
    | grep -m1 '"tag_name"' | cut -d'"' -f4)" || ver=""

  if [ -n "$ver" ]; then
    url="https://github.com/crate-ci/typos/releases/download/${ver}/typos-${ver}-x86_64-unknown-linux-musl.tar.gz"
    tmp="$(mktemp -d)"
    if curl -fsSL "$url" | tar xz -C "$tmp" 2>/dev/null && [ -x "$tmp/typos" ]; then
      mkdir -p "$HOME/.local/bin"
      mv "$tmp/typos" "$HOME/.local/bin/typos"
      rm -rf "$tmp"
      log "typos: installed ${ver} to ~/.local/bin"
      return
    fi
    rm -rf "$tmp"
    log "typos: prebuilt binary unavailable, falling back to cargo install"
  fi

  cargo install typos-cli
  log "typos: done (cargo install)"
}

setup_git_hooks() {
  log "git hooks: start"
  if [ -d .githooks ]; then
    git config core.hooksPath .githooks
    chmod +x .githooks/* 2>/dev/null || true
    log "git hooks: core.hooksPath -> .githooks"
  else
    log "git hooks: skip (.githooks not found)"
  fi
}

install_claude() {
  log "claude: start"
  if command -v claude >/dev/null 2>&1; then
    log "claude: already installed"
    return
  fi
  curl -fsSL https://claude.ai/install.sh | bash
  log "claude: done"
}

main() {
  install_rust_components
  install_typos
  setup_git_hooks
  install_claude
}

main "$@"
