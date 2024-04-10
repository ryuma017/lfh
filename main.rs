#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(not(target_os = "macos"))] {
        compile_error!("This program is only intended to run on macOS.");
    }
}

use std::process::Command;

use anyhow::{ensure, Context as _, Result};
use icrate::{AppKit::NSRunningApplication, Foundation::NSString};
use objc2::rc::Id;

const USAGE: &str = "Usage: appctl <bundle-id>";

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    let bundle_id = match args.pop() {
        None => anyhow::bail!("{USAGE}"),
        Some(bundle_id) => {
            anyhow::ensure!(args.is_empty(), "{USAGE}");
            bundle_id
        }
    };

    let app = RunningApp::find(&bundle_id);

    match app {
        None => {
            // It's not running, launch it with open command.
            open(&bundle_id)?;
        }
        Some(runninng_app) => {
            if runninng_app.is_active() {
                // It's active in the frontmost, hide it.
                runninng_app.hide()?;
            } else {
                // It may be hidden, behind another application, running in the
                // background, or not running. Leave it to the open command to
                // make them appear in frontmost.
                open(&bundle_id)?;
            }
        }
    }

    Ok(())
}

fn open(bundle_id: &str) -> Result<()> {
    const PROGRAM: &str = "open";

    let resolved = grep_cli::resolve_binary(PROGRAM)
        .with_context(|| format!("resolve a path to `{}`", PROGRAM))?;

    Command::new(&resolved)
        .arg("-b")
        .arg(bundle_id)
        .spawn()
        .with_context(|| format!("{} -b {}", resolved.display(), bundle_id))?;

    Ok(())
}

struct RunningApp(Id<NSRunningApplication>);

impl RunningApp {
    fn find(bundle_id: &str) -> Option<Self> {
        let id_nsstr = NSString::from_str(bundle_id);
        unsafe {
            NSRunningApplication::runningApplicationsWithBundleIdentifier(
                &id_nsstr,
            )
        }
        .into_iter()
        .next()
        .map(RunningApp)
    }

    fn is_active(&self) -> bool {
        unsafe { self.0.isActive() }
    }

    fn hide(&self) -> Result<()> {
        ensure!(unsafe { self.0.hide() }, "hide the application");
        Ok(())
    }
}
