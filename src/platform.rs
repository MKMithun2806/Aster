#![allow(dead_code)]
//! Platform abstraction for Aster.
//!
//! The current Windows backend remains native Win32/WebView2 for stability.
//! Linux will route through a tao + wry shell that keeps the browser state
//! model intact while moving the chrome UI into HTML/CSS/JS where needed.
//!
//! This module is intentionally small and explicit so the browser state can be
//! shared while the platform-specific shell stays isolated behind `cfg` gates.

use std::error::Error;

/// Result type used by platform backends.
pub type PlatformResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// Minimal browser actions that both backends need to expose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformCommand {
    CreateTab { url: String },
    NavigateActive { url: String },
    CloseActiveTab,
    GoBack,
    GoForward,
    Reload,
    SwitchWorkspace { workspace_id: usize },
    ToggleSidebar,
}

/// Messages emitted by the UI layer back to Rust.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcMessage {
    Command(PlatformCommand),
    RequestState,
    StateSynced,
}

/// Common window operations used by shell backends.
pub trait WindowHost {
    fn set_title(&self, title: &str) -> PlatformResult<()>;
    fn set_visible(&self, visible: bool) -> PlatformResult<()>;
    fn resize(&self, width: i32, height: i32) -> PlatformResult<()>;
}

/// Common webview operations used by shell backends.
pub trait WebViewHost {
    fn load_url(&self, url: &str) -> PlatformResult<()>;
    fn reload(&self) -> PlatformResult<()>;
    fn go_back(&self) -> PlatformResult<()>;
    fn go_forward(&self) -> PlatformResult<()>;
    fn evaluate_script(&self, script: &str) -> PlatformResult<()>;
}

/// Backends implement this trait to own the event loop and IPC dispatch.
pub trait PlatformApp {
    fn run(self) -> PlatformResult<()>;
}
