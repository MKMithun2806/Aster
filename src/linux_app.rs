use std::{error::Error, path::PathBuf};

use serde::{Deserialize, Serialize};
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy},
    window::WindowBuilder,
};
use wry::{http::Request, WebView, WebViewBuilder};

const DEFAULT_URL: &str = "https://www.google.com";
const STATE_FILE: &str = ".aster-state";

#[derive(Debug, Clone, Serialize)]
struct ShellState {
    active_index: usize,
    active_url: String,
    tabs: Vec<ShellTab>,
}

#[derive(Debug, Clone, Serialize)]
struct ShellTab {
    id: usize,
    title: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ShellIpc {
    Navigate { url: String },
    NewTab { url: Option<String> },
    SwitchTab { index: usize },
    CloseTab,
    Back,
    Forward,
    Reload,
    RequestSync,
}

enum UserEvent {
    Ipc(String),
}

struct LinuxBrowser {
    tabs: Vec<ShellTab>,
    active_index: usize,
    next_tab_id: usize,
    state_path: PathBuf,
}

impl LinuxBrowser {
    fn new() -> Self {
        let state_path = state_path();
        let mut browser = Self {
            tabs: Vec::new(),
            active_index: 0,
            next_tab_id: 1,
            state_path,
        };
        browser.load_state();
        if browser.tabs.is_empty() {
            browser.tabs.push(ShellTab {
                id: 1,
                title: "New Tab".to_string(),
                url: DEFAULT_URL.to_string(),
            });
            browser.active_index = 0;
            browser.next_tab_id = 2;
        }
        browser
    }

    fn active_tab(&self) -> Option<&ShellTab> {
        self.tabs.get(self.active_index)
    }

    fn active_tab_mut(&mut self) -> Option<&mut ShellTab> {
        self.tabs.get_mut(self.active_index)
    }

    fn create_tab(&mut self, url: String) {
        let tab = ShellTab {
            id: self.next_tab_id,
            title: "New Tab".to_string(),
            url: normalize_url(&url),
        };
        self.next_tab_id += 1;
        self.tabs.push(tab);
        self.active_index = self.tabs.len().saturating_sub(1);
        self.save_state();
    }

    fn close_active_tab(&mut self) {
        if self.tabs.len() <= 1 {
            if let Some(tab) = self.active_tab_mut() {
                tab.url = DEFAULT_URL.to_string();
                tab.title = "New Tab".to_string();
            }
            self.active_index = 0;
            self.save_state();
            return;
        }

        if self.active_index < self.tabs.len() {
            self.tabs.remove(self.active_index);
            if self.active_index >= self.tabs.len() {
                self.active_index = self.tabs.len().saturating_sub(1);
            }
        }
        self.save_state();
    }

    fn switch_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_index = index;
            self.save_state();
        }
    }

    fn navigate_active(&mut self, url: String) {
        if let Some(tab) = self.active_tab_mut() {
            tab.url = normalize_url(&url);
            tab.title = label_for_url(&tab.url);
        }
        self.save_state();
    }

    fn sync_state(&self) -> ShellState {
        ShellState {
            active_index: self.active_index,
            active_url: self
                .active_tab()
                .map(|tab| tab.url.clone())
                .unwrap_or_default(),
            tabs: self.tabs.clone(),
        }
    }

    fn load_state(&mut self) {
        let Ok(raw) = std::fs::read_to_string(&self.state_path) else {
            return;
        };

        let mut tabs = Vec::new();
        let mut active_workspace = 1usize;
        let mut active_tab_id = None;

        for line in raw.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            match parts.as_slice() {
                ["active_workspace", id] => {
                    active_workspace = id.parse::<usize>().unwrap_or(1);
                }
                ["active_tab", workspace_id, tab_id] => {
                    if workspace_id.parse::<usize>().unwrap_or(1) == active_workspace {
                        active_tab_id = tab_id.parse::<usize>().ok();
                    }
                }
                ["tab", workspace_id, _folder_id, _pinned, title, url, ..]
                    if workspace_id.parse::<usize>().unwrap_or(1) == active_workspace =>
                {
                    tabs.push(ShellTab {
                        id: tabs.len() + 1,
                        title: unescape_state(title),
                        url: unescape_state(url),
                    });
                }
                _ => {}
            }
        }

        if !tabs.is_empty() {
            self.next_tab_id = tabs.len() + 1;
            if let Some(id) = active_tab_id {
                if let Some(index) = tabs.iter().position(|tab| tab.id == id) {
                    self.active_index = index;
                }
            }
            self.tabs = tabs;
        }
    }

    fn save_state(&self) {
        let mut lines = Vec::new();
        lines.push("active_workspace\t1".to_string());
        lines.push(format!("workspace\t1\t{}", escape_state("Space 1")));

        if let Some(active) = self.active_tab() {
            lines.push(format!("active_tab\t1\t{}", active.id));
        }

        for tab in &self.tabs {
            lines.push(format!(
                "tab\t1\t\t0\t{}\t{}\t",
                escape_state(&tab.title),
                escape_state(&tab.url)
            ));
        }

        if let Some(parent) = self.state_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&self.state_path, lines.join("\n"));
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let event_loop: EventLoop<UserEvent> = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_title("Aster")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 900.0))
        .build(&event_loop)?;

    let mut browser = LinuxBrowser::new();
    let initial_url = browser
        .active_tab()
        .map(|tab| tab.url.clone())
        .unwrap_or_else(|| DEFAULT_URL.to_string());

    let mut webview = build_webview(&window, &proxy, &initial_url)?;
    sync_shell(&mut webview, &browser)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                *control_flow = ControlFlow::Wait;
            }
            Event::UserEvent(UserEvent::Ipc(payload)) => {
                if let Ok(message) = serde_json::from_str::<ShellIpc>(&payload) {
                    match message {
                        ShellIpc::Navigate { url } => {
                            browser.navigate_active(url.clone());
                            let _ = webview.load_url(&normalize_url(&url));
                            let _ = sync_shell(&mut webview, &browser);
                        }
                        ShellIpc::NewTab { url } => {
                            browser.create_tab(url.unwrap_or_else(|| DEFAULT_URL.to_string()));
                            let url = browser
                                .active_tab()
                                .map(|tab| tab.url.clone())
                                .unwrap_or_else(|| DEFAULT_URL.to_string());
                            let _ = webview.load_url(&url);
                            let _ = sync_shell(&mut webview, &browser);
                        }
                        ShellIpc::SwitchTab { index } => {
                            browser.switch_tab(index);
                            if let Some(tab) = browser.active_tab() {
                                let _ = webview.load_url(&tab.url);
                            }
                            let _ = sync_shell(&mut webview, &browser);
                        }
                        ShellIpc::CloseTab => {
                            browser.close_active_tab();
                            if let Some(tab) = browser.active_tab() {
                                let _ = webview.load_url(&tab.url);
                            }
                            let _ = sync_shell(&mut webview, &browser);
                        }
                        ShellIpc::Back => {
                            let _ = webview.evaluate_script("window.history.back();");
                        }
                        ShellIpc::Forward => {
                            let _ = webview.evaluate_script("window.history.forward();");
                        }
                        ShellIpc::Reload => {
                            let _ = webview.reload();
                        }
                        ShellIpc::RequestSync => {
                            let _ = sync_shell(&mut webview, &browser);
                        }
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

fn build_webview(
    window: &tao::window::Window,
    proxy: &EventLoopProxy<UserEvent>,
    initial_url: &str,
) -> Result<WebView, Box<dyn Error>> {
    let initialization_script = shell_script();
    let webview = WebViewBuilder::new()
        .with_url(initial_url)
        .with_initialization_script(&initialization_script)
        .with_on_page_load_handler({
            let proxy = proxy.clone();
            move |event, _url| {
                if matches!(event, wry::PageLoadEvent::Finished) {
                    if let Ok(payload) = serde_json::to_string(&ShellIpc::RequestSync) {
                        let _ = proxy.send_event(UserEvent::Ipc(payload));
                    }
                }
            }
        })
        .with_ipc_handler({
            let proxy = proxy.clone();
            move |request: Request<String>| {
                let _ = proxy.send_event(UserEvent::Ipc(request.body().clone()));
            }
        })
        .build(window)?;
    Ok(webview)
}

fn sync_shell(webview: &mut WebView, browser: &LinuxBrowser) -> Result<(), Box<dyn Error>> {
    let state = browser.sync_state();
    let state_json = serde_json::to_string(&state)?;
    webview.evaluate_script(&format!("window.AsterSync({state_json});"))?;
    Ok(())
}

fn shell_script() -> String {
    r#"
(function () {
  const rootId = 'aster-shell-root';
  const sidebarWidth = 252;
  const topbarHeight = 56;

  function ensureRoot() {
    let root = document.getElementById(rootId);
    if (!root) {
      root = document.createElement('div');
      root.id = rootId;
      root.innerHTML = `
        <style>
          #${rootId} {
            position: fixed;
            inset: 0;
            z-index: 2147483647;
            pointer-events: none;
            font-family: Inter, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
          }
          #${rootId} .panel {
            pointer-events: auto;
            position: fixed;
            top: 0;
            left: 0;
            width: ${sidebarWidth}px;
            height: 100vh;
            background: linear-gradient(180deg, rgba(8,8,8,.96), rgba(17,17,17,.96));
            color: #f5f5f5;
            border-right: 1px solid rgba(255,255,255,.08);
            box-shadow: 12px 0 40px rgba(0,0,0,.26);
            display: flex;
            flex-direction: column;
          }
          #${rootId} .topbar {
            height: ${topbarHeight}px;
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 12px;
            box-sizing: border-box;
            border-bottom: 1px solid rgba(255,255,255,.08);
          }
          #${rootId} .brand {
            font-weight: 700;
            letter-spacing: .04em;
            text-transform: uppercase;
            opacity: .88;
          }
          #${rootId} .nav {
            display: flex;
            gap: 6px;
          }
          #${rootId} button {
            border: 0;
            border-radius: 10px;
            background: rgba(255,255,255,.08);
            color: inherit;
            padding: 8px 10px;
            cursor: pointer;
          }
          #${rootId} button:hover {
            background: rgba(255,255,255,.14);
          }
          #${rootId} .address {
            width: 100%;
            margin: 0 12px 8px;
            box-sizing: border-box;
            border-radius: 12px;
            border: 1px solid rgba(255,255,255,.1);
            background: rgba(255,255,255,.06);
            color: #fff;
            padding: 10px 12px;
            outline: none;
          }
          #${rootId} .section-title {
            padding: 12px 12px 6px;
            text-transform: uppercase;
            letter-spacing: .08em;
            font-size: 11px;
            color: rgba(245,245,245,.55);
          }
          #${rootId} .tabs {
            display: flex;
            flex-direction: column;
            gap: 8px;
            padding: 0 10px 12px;
            overflow: auto;
          }
          #${rootId} .tab {
            border-radius: 14px;
            background: rgba(255,255,255,.05);
            border: 1px solid rgba(255,255,255,.08);
            padding: 10px 12px;
            text-align: left;
          }
          #${rootId} .tab.active {
            background: rgba(241,111,99,.18);
            border-color: rgba(241,111,99,.5);
          }
          #${rootId} .tab-title {
            font-size: 13px;
            font-weight: 600;
            display: block;
            margin-bottom: 4px;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
          }
          #${rootId} .tab-url {
            font-size: 12px;
            color: rgba(245,245,245,.66);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
          }
          #${rootId} .content-guard {
            position: fixed;
            left: ${sidebarWidth}px;
            top: 0;
            right: 0;
            height: ${topbarHeight}px;
            pointer-events: none;
          }
          html, body {
            margin-left: ${sidebarWidth}px !important;
            margin-top: ${topbarHeight}px !important;
          }
        </style>
        <div class="panel">
          <div class="topbar">
            <div class="brand">Aster</div>
            <div class="nav">
              <button data-action="back">Back</button>
              <button data-action="forward">Forward</button>
              <button data-action="reload">Reload</button>
              <button data-action="new-tab">New Tab</button>
            </div>
          </div>
          <input class="address" type="text" placeholder="Search or enter URL" />
          <div class="section-title">Tabs</div>
          <div class="tabs"></div>
        </div>
      `;
      (document.documentElement || document.body).appendChild(root);

      const address = root.querySelector('.address');
      const send = (msg) => window.ipc.postMessage(JSON.stringify(msg));
      root.addEventListener('click', (event) => {
        const target = event.target.closest('button[data-action]');
        if (!target) return;
        const action = target.dataset.action;
        if (action === 'back' || action === 'forward' || action === 'reload') {
          send({ type: action });
        } else if (action === 'new-tab') {
          send({ type: 'new_tab', url: address?.value || '' });
        }
      });
      address?.addEventListener('keydown', (event) => {
        if (event.key === 'Enter') {
          send({ type: 'navigate', url: address.value });
        }
      });
      window.AsterSync = function (state) {
        const tabs = root.querySelector('.tabs');
        if (!tabs || !state) return;
        tabs.innerHTML = '';
        (state.tabs || []).forEach((tab, index) => {
          const item = document.createElement('button');
          item.className = 'tab' + (index === state.active_index ? ' active' : '');
          item.innerHTML = `
            <span class="tab-title">${escapeHtml(tab.title || 'New Tab')}</span>
            <span class="tab-url">${escapeHtml(tab.url || '')}</span>
          `;
          item.addEventListener('click', () => send({ type: 'switch_tab', index }));
          item.addEventListener('auxclick', (event) => {
            if (event.button === 1) {
              send({ type: 'close_tab' });
            }
          });
          tabs.appendChild(item);
        });
        if (address && state.active_url !== undefined) {
          address.value = state.active_url || '';
        }
      };
      function escapeHtml(value) {
        return String(value)
          .replaceAll('&', '&amp;')
          .replaceAll('<', '&lt;')
          .replaceAll('>', '&gt;')
          .replaceAll('"', '&quot;')
          .replaceAll("'", '&#39;');
      }
    }
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', ensureRoot, { once: true });
  } else {
    ensureRoot();
  }
})();
"#
    .to_string()
}

fn state_path() -> PathBuf {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("Aster").join(STATE_FILE)
}

fn normalize_url(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return DEFAULT_URL.to_string();
    }
    if trimmed.contains("://") {
        trimmed.to_string()
    } else {
        format!("https://{trimmed}")
    }
}

fn label_for_url(url: &str) -> String {
    let normalized = normalize_url(url);
    if normalized == DEFAULT_URL {
        "New Tab".to_string()
    } else {
        normalized
    }
}

fn escape_state(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn unescape_state(value: &str) -> String {
    let mut out = String::new();
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('t') => out.push('\t'),
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('\\') => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}
