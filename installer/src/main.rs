#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod patch;

use patch::{find_imports, find_installs, import_settings, install, kill, uninstall, Install};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

fn brand(dir_name: &str) -> i32 {
    match dir_name {
        "DiscordCanary" => 2,
        "DiscordPTB" => 1,
        _ => 0,
    }
}

fn rows(installs: &[Install]) -> ModelRc<Row> {
    ModelRc::new(VecModel::from(
        installs
            .iter()
            .map(|i| {
                let (kind, state) = match (&i.other, i.patched) {
                    (Some(other), _) => (2, format!("{other} is installed")),
                    (None, true) => (1, "romantic is installed".to_owned()),
                    _ => (0, "No mod installed".to_owned()),
                };
                Row {
                    name: i.name.clone().into(),
                    state: state.into(),
                    version: format!("v{}", i.version).into(),
                    kind,
                    brand: brand(&i.dir_name),
                }
            })
            .collect::<Vec<_>>(),
    ))
}

fn detail(installs: &[Install], selected: usize) -> String {
    match installs.get(selected) {
        Some(i) => format!("{}\n{}", i.name, i.resources.display()),
        None => "Nothing selected".to_owned(),
    }
}

fn import_label() -> String {
    let found = find_imports();
    match found.first() {
        Some((name, _)) => format!("Import my settings from {name}"),
        None => String::new(),
    }
}

struct Log(Vec<LogLine>);

impl Log {
    fn push(&mut self, window: &App, text: impl Into<String>, kind: i32) {
        self.0.push(LogLine { text: text.into().into(), kind });
        let start = self.0.len().saturating_sub(7);
        window.set_log(ModelRc::new(VecModel::from(self.0[start..].to_vec())));
    }
}

fn stamp() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    format!("{:02}:{:02}:{:02}", secs / 3600 % 24, secs / 60 % 60, secs % 60)
}

fn cli(args: &[String]) -> Option<i32> {
    let action = args
        .iter()
        .find(|a| matches!(a.as_str(), "--install" | "--repair" | "--uninstall" | "--list"))?;
    let installs = find_installs();
    if action == "--list" {
        for i in &installs {
            println!("{} patched={} other={:?} {}", i.name, i.patched, i.other, i.resources.display());
        }
        return Some(0);
    }
    let branch = args.iter().position(|a| a == "--branch").and_then(|p| args.get(p + 1));
    let mut code = 0;
    for i in installs
        .iter()
        .filter(|i| branch.is_none_or(|b| i.dir_name.eq_ignore_ascii_case(b)))
    {
        if std::env::var_os("ROMANTIC_NOKILL").is_none() {
            kill(&i.dir_name);
        }
        let result = if action == "--uninstall" { uninstall(i) } else { install(i) };
        match result {
            Ok(()) => println!("{}: ok", i.name),
            Err(e) => {
                eprintln!("{}: {e}", i.name);
                code = 1;
            }
        }
    }
    Some(code)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(code) = cli(&args) {
        std::process::exit(code);
    }
    let app = App::new().unwrap();
    let installs = Rc::new(RefCell::new(find_installs()));
    let log = Rc::new(RefCell::new(Log(Vec::new())));

    app.set_build(concat!("v", env!("CARGO_PKG_VERSION")).into());
    app.set_imports(import_label().into());
    app.set_rows(rows(&installs.borrow()));
    app.set_detail(detail(&installs.borrow(), 0).into());
    log.borrow_mut().push(
        &app,
        format!("{} - Found {} Discord client(s)", stamp(), installs.borrow().len()),
        0,
    );

    let weak = app.as_weak();
    let list = installs.clone();
    app.on_selected_changed(move || {
        if let Some(app) = weak.upgrade() {
            app.set_detail(detail(&list.borrow(), app.get_selected() as usize).into());
        }
    });

    let weak = app.as_weak();
    let list = installs.clone();
    let logs = log.clone();
    app.on_rescan(move || {
        *list.borrow_mut() = find_installs();
        if let Some(app) = weak.upgrade() {
            app.set_rows(rows(&list.borrow()));
            app.set_selected(0);
            app.set_imports(import_label().into());
            app.set_detail(detail(&list.borrow(), 0).into());
            logs.borrow_mut().push(&app, format!("{} - Rescanned", stamp()), 0);
        }
    });

    for removing in [false, true] {
        let weak = app.as_weak();
        let list = installs.clone();
        let logs = log.clone();
        let handler = move || {
            let Some(app) = weak.upgrade() else { return };
            let index = app.get_selected() as usize;
            let Some(target) = list.borrow().get(index).cloned() else { return };
            let mut logs = logs.borrow_mut();
            logs.push(&app, format!("{} - Closing {}", stamp(), target.name), 0);
            kill(&target.dir_name);
            let result = if removing { uninstall(&target) } else { install(&target) };
            match result {
                Ok(()) if removing => {
                    logs.push(&app, format!("{} - {} is back to normal", stamp(), target.name), 1)
                }
                Ok(()) => {
                    logs.push(&app, format!("{} - Patched {}", stamp(), target.name), 1);
                    if app.get_do_import() && !app.get_imports().is_empty() {
                        match import_settings() {
                            Ok(note) => logs.push(&app, format!("{} - {note}", stamp()), 1),
                            Err(e) => logs.push(&app, format!("{} - Import failed: {e}", stamp()), 2),
                        }
                    }
                    logs.push(&app, format!("{} - Open Discord and find romantic in Settings", stamp()), 0);
                }
                Err(e) => logs.push(&app, format!("{} - {e}", stamp()), 2),
            }
            drop(logs);
            *list.borrow_mut() = find_installs();
            app.set_rows(rows(&list.borrow()));
            app.set_detail(detail(&list.borrow(), index.min(list.borrow().len().saturating_sub(1))).into());
        };
        if removing {
            app.on_uninstall(handler);
        } else {
            app.on_install(handler);
        }
    }
    let _ = SharedString::new();
    app.run().unwrap();
}
