#!/usr/bin/env bash
set -euo pipefail

repo_url="https://github.com/ahyanistheEmty/Aster.git"
binary_name="Aster"
desktop_name="aster.desktop"

log() {
  printf '%s\n' "$*"
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "required command '$1' was not found"
}

install_packages_apt() {
  log "Installing Linux build dependencies with apt..."
  if [[ $EUID -eq 0 ]]; then
    apt-get update
    local webkit_pkg="libwebkit2gtk-4.1-dev"
    if ! apt-cache show "$webkit_pkg" >/dev/null 2>&1; then
      webkit_pkg="libwebkit2gtk-4.0-dev"
    fi
    apt-get install -y \
      build-essential \
      curl \
      git \
      pkg-config \
      libgtk-3-dev \
      "$webkit_pkg"
  else
    sudo apt-get update
    local webkit_pkg="libwebkit2gtk-4.1-dev"
    if ! apt-cache show "$webkit_pkg" >/dev/null 2>&1; then
      webkit_pkg="libwebkit2gtk-4.0-dev"
    fi
    sudo apt-get install -y \
      build-essential \
      curl \
      git \
      pkg-config \
      libgtk-3-dev \
      "$webkit_pkg"
  fi
}

install_packages_pacman() {
  local packages=(
    base-devel
    curl
    git
    pkgconf
    gtk3
    webkit2gtk
  )

  log "Installing Linux build dependencies with pacman..."
  if [[ $EUID -eq 0 ]]; then
    pacman -Sy --needed --noconfirm "${packages[@]}"
  else
    sudo pacman -Sy --needed --noconfirm "${packages[@]}"
  fi
}

install_system_deps() {
  if command -v apt-get >/dev/null 2>&1; then
    install_packages_apt
  elif command -v pacman >/dev/null 2>&1; then
    install_packages_pacman
  else
    log "No supported package manager detected; skipping system dependency install."
    log "Install build tools, pkg-config, GTK, and WebKit2GTK manually before continuing."
  fi
}

resolve_source_dir() {
  if [[ -f Cargo.toml ]]; then
    printf '%s\n' "$PWD"
    return
  fi

  local tmp_dir
  tmp_dir="$(mktemp -d -t "aster-install-XXXXXX")"
  log "Cloning Aster into temporary directory: $tmp_dir"
  git clone --depth 1 "$repo_url" "$tmp_dir" >/dev/null
  printf '%s\n' "$tmp_dir"
}

install_binary() {
  local source_dir="$1"
  local install_dir="${ASTER_INSTALL_DIR:-}"
  if [[ -z "$install_dir" ]]; then
    if [[ $EUID -eq 0 ]]; then
      install_dir="/usr/local/bin"
    else
      install_dir="$HOME/.local/bin"
    fi
  fi
  local desktop_dir="${XDG_DATA_HOME:-$HOME/.local/share}/applications"
  local icon_dir="${XDG_DATA_HOME:-$HOME/.local/share}/icons/hicolor/scalable/apps"
  local target_bin="${install_dir}/${binary_name}"
  local desktop_file="${desktop_dir}/${desktop_name}"
  local icon_source="${source_dir}/assets/aster-star.svg"
  local icon_target="${icon_dir}/aster.svg"
  local desktop_exec="${target_bin// /\\ }"

  mkdir -p "$install_dir" "$desktop_dir" "$icon_dir"

  log "Building Aster in release mode..."
  cargo build --release --manifest-path "$source_dir/Cargo.toml"

  install -m 0755 "$source_dir/target/release/$binary_name" "$target_bin"

  if [[ -f "$icon_source" ]]; then
    install -m 0644 "$icon_source" "$icon_target"
  fi

  cat >"$desktop_file" <<EOF
[Desktop Entry]
Type=Application
Name=Aster Browser
Exec=${desktop_exec} %U
Icon=aster
Terminal=false
Categories=Network;WebBrowser;
StartupNotify=true
EOF

  if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$desktop_dir" >/dev/null 2>&1 || true
  fi

  log ""
  log "Installation complete."
  log "Binary: $target_bin"
  log "Desktop file: $desktop_file"
  log "State: ${XDG_CONFIG_HOME:-$HOME/.config}/Aster"
}

main() {
  need_cmd git
  need_cmd cargo
  install_system_deps

  local source_dir
  source_dir="$(resolve_source_dir)"

  if [[ -d "$source_dir" && "$source_dir" != "$PWD" ]]; then
    trap 'rm -rf "$source_dir"' EXIT
  fi

  install_binary "$source_dir"
}

main "$@"
