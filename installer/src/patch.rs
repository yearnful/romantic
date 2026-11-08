use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub const FILES: [(&str, &[u8]); 5] = [
    ("package.json", include_bytes!("../assets/package.json")),
    ("patcher.js", include_bytes!("../assets/patcher.js")),
    ("preload.js", include_bytes!("../assets/preload.js")),
    ("renderer.js", include_bytes!("../assets/renderer.js")),
    ("renderer.css", include_bytes!("../assets/renderer.css")),
];

pub const BRANCHES: [(&str, &str); 3] = [
    ("Discord", "Discord"),
    ("Discord PTB", "DiscordPTB"),
    ("Discord Canary", "DiscordCanary"),
];

#[derive(Clone, Debug)]
pub struct Install {
    pub name: String,
    pub dir_name: String,
    pub root: PathBuf,
    pub resources: PathBuf,
    pub patched: bool,
    pub other: Option<String>,
    pub version: String,
}

pub fn local_root() -> PathBuf {
    std::env::var_os("ROMANTIC_LOCAL")
        .or_else(|| std::env::var_os("LOCALAPPDATA"))
        .map(PathBuf::from)
        .unwrap_or_default()
}

pub fn data_dir() -> PathBuf {
    std::env::var_os("ROMANTIC_DATA")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("APPDATA").map(|p| PathBuf::from(p).join("romantic")))
        .unwrap_or_else(|| PathBuf::from("romantic"))
}

fn version_of(name: &str) -> Option<Vec<u32>> {
    let rest = name.strip_prefix("app-")?;
    rest.split('.').map(|part| part.parse().ok()).collect()
}

pub fn find_installs() -> Vec<Install> {
    let base = local_root();
    let mut found = Vec::new();
    for (name, dir_name) in BRANCHES {
        let root = base.join(dir_name);
        let Ok(entries) = fs::read_dir(&root) else {
            continue;
        };
        let mut best: Option<(Vec<u32>, PathBuf)> = None;
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().into_owned();
            let Some(version) = version_of(&file_name) else {
                continue;
            };
            let resources = entry.path().join("resources");
            if !resources.join("app.asar").exists() && !resources.join("_app.asar").exists() {
                continue;
            }
            if best.as_ref().is_none_or(|(old, _)| version > *old) {
                best = Some((version, resources));
            }
        }
        if let Some((_, resources)) = best {
            let patched = resources.join("_app.asar").exists();
            let other = if patched { other_mod(&resources.join("app.asar")) } else { None };
            let version = resources
                .parent()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().trim_start_matches("app-").to_owned())
                .unwrap_or_default();
            found.push(Install {
                name: name.to_owned(),
                dir_name: dir_name.to_owned(),
                root,
                resources,
                patched: patched && other.is_none(),
                other,
                version,
            });
        }
    }
    found
}

pub fn write_loader_asar(out: &Path, patcher: &Path) -> std::io::Result<()> {
    let path = serde_json::to_string(&patcher.to_string_lossy()).unwrap();
    let index = format!("require({path})");
    let package = r#"{"name":"discord","main":"index.js"}"#;
    let header = serde_json::json!({
        "files": {
            "index.js": { "size": index.len(), "offset": "0" },
            "package.json": { "size": package.len(), "offset": index.len().to_string() },
        }
    })
    .to_string();
    let size = header.len() as u32;
    let aligned = (size + 3) & !3;
    let mut padded = header;
    padded.push_str(&"0".repeat((aligned - size) as usize));
    let mut file = fs::File::create(out)?;
    for n in [4u32, aligned + 8, aligned + 4, size] {
        file.write_all(&n.to_le_bytes())?;
    }
    file.write_all(padded.as_bytes())?;
    file.write_all(index.as_bytes())?;
    file.write_all(package.as_bytes())?;
    Ok(())
}

pub fn put_files() -> Result<PathBuf, String> {
    let dir = data_dir().join("dist");
    fs::create_dir_all(&dir).map_err(|e| format!("Could not create {}: {e}", dir.display()))?;
    for (name, bytes) in FILES {
        fs::write(dir.join(name), bytes).map_err(|e| format!("Could not write {name}: {e}"))?;
    }
    Ok(dir.join("patcher.js"))
}

fn other_mod(app_asar: &Path) -> Option<String> {
    let bytes = fs::read(app_asar).ok()?;
    if bytes.len() > 4096 {
        return None;
    }
    let text = String::from_utf8_lossy(&bytes).to_lowercase();
    if !text.contains("require(") || text.contains("romantic") {
        return None;
    }
    for name in ["equicord", "vencord", "vesktop", "betterdiscord", "replugged"] {
        if text.contains(name) {
            let mut n = name.to_owned();
            n[..1].make_ascii_uppercase();
            return Some(n);
        }
    }
    Some("Another mod".to_owned())
}

fn is_loader(app_asar: &Path) -> bool {
    fs::read(app_asar)
        .map(|b| b.windows(8).any(|w| w == b"require("))
        .unwrap_or(false)
        && fs::metadata(app_asar).map(|m| m.len() < 4096).unwrap_or(false)
}

fn busy(e: std::io::Error) -> String {
    if e.kind() == std::io::ErrorKind::PermissionDenied || e.raw_os_error() == Some(32) {
        "Discord is still running. Close it fully (check the tray) and try again.".into()
    } else {
        e.to_string()
    }
}

pub fn kill(dir_name: &str) {
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("taskkill")
            .args(["/F", "/IM", &format!("{dir_name}.exe")])
            .output();
        std::thread::sleep(std::time::Duration::from_millis(800));
    }
    #[cfg(not(windows))]
    let _ = dir_name;
}

pub fn install(target: &Install) -> Result<(), String> {
    let patcher = put_files()?;
    let app = target.resources.join("app.asar");
    let original = target.resources.join("_app.asar");
    if original.exists() {
        if is_loader(&app) {
            write_loader_asar(&app, &patcher).map_err(|e| busy(e))?;
            return Ok(());
        }
        fs::remove_file(&original).map_err(busy)?;
    }
    fs::rename(&app, &original).map_err(busy)?;
    if let Err(e) = write_loader_asar(&app, &patcher) {
        let _ = fs::remove_file(&app);
        let _ = fs::rename(&original, &app);
        return Err(busy(e));
    }
    Ok(())
}

pub fn uninstall(target: &Install) -> Result<(), String> {
    let app = target.resources.join("app.asar");
    let original = target.resources.join("_app.asar");
    if !original.exists() {
        return Ok(());
    }
    if is_loader(&app) {
        fs::remove_file(&app).map_err(busy)?;
    }
    fs::rename(&original, &app).map_err(busy)?;
    Ok(())
}

pub fn find_imports() -> Vec<(String, PathBuf)> {
    let Some(appdata) = std::env::var_os("ROMANTIC_ROAMING")
        .or_else(|| std::env::var_os("APPDATA"))
        .map(PathBuf::from)
    else {
        return Vec::new();
    };
    ["Vencord", "Equicord"]
        .iter()
        .map(|n| ((*n).to_owned(), appdata.join(n)))
        .filter(|(_, dir)| dir.join("settings").join("settings.json").exists())
        .collect()
}

fn copy_dir(from: &Path, to: &Path) -> std::io::Result<u32> {
    fs::create_dir_all(to)?;
    let mut count = 0;
    for entry in fs::read_dir(from)?.flatten() {
        let path = entry.path();
        let target = to.join(entry.file_name());
        if path.is_dir() {
            count += copy_dir(&path, &target)?;
        } else {
            fs::copy(&path, &target)?;
            count += 1;
        }
    }
    Ok(count)
}

pub fn import_settings() -> Result<String, String> {
    let Some((name, source)) = find_imports().into_iter().next() else {
        return Err("No settings to import".into());
    };
    let dest = data_dir();
    let settings = dest.join("settings");
    fs::create_dir_all(&settings).map_err(|e| e.to_string())?;
    let current = settings.join("settings.json");
    if current.exists() {
        let _ = fs::copy(&current, settings.join("settings.before-import.json"));
    }
    let mut files = 0;
    for file in ["settings.json", "quickCss.css"] {
        let from = source.join("settings").join(file);
        if from.exists() {
            fs::copy(&from, settings.join(file)).map_err(|e| e.to_string())?;
            files += 1;
        }
    }
    let themes = source.join("themes");
    if themes.exists() {
        files += copy_dir(&themes, &dest.join("themes")).map_err(|e| e.to_string())?;
    }
    Ok(format!("Imported {files} file(s) from {name}. The old folder is untouched."))
}
