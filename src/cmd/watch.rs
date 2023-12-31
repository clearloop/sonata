//! command new

use crate::App;
use anyhow::Result;
use ccli::{clap, clap::Parser};
use notify::{
    event::{Event, EventKind, MetadataKind, ModifyKind},
    Watcher,
};
use std::{
    path::PathBuf,
    sync::mpsc::{self, Sender},
    thread,
};

/// Watch command
#[derive(Parser, Debug)]
pub struct Watch {
    /// The root of the cydonia site
    #[clap(default_value = ".")]
    pub dir: PathBuf,

    /// The output directory
    #[clap(short, long, default_value = "out")]
    pub out: PathBuf,
}

impl Watch {
    /// Watch the given directory.
    pub fn watch(&self, tx: Sender<Event>) -> Result<()> {
        let mut app = App::load(&self.dir)?;
        if self.out.is_absolute() {
            app.manifest.out = self.out.clone();
        } else if self.out != PathBuf::from("out") {
            let out = self.dir.join("out");
            app.manifest.out = out.clone();
        }

        app.render()?;
        tracing::info!(
            "watching {} -> {}",
            self.dir.display(),
            app.manifest.out.display()
        );

        let cloned_app = app.clone();
        let mut watcher =
            notify::recommended_watcher(move |res: notify::Result<Event>| match res {
                Ok(event) => {
                    tracing::trace!("event: {:#?}", event);
                    if event.kind == EventKind::Modify(ModifyKind::Metadata(MetadataKind::Any)) {
                        return;
                    }

                    if let Err(e) = cloned_app.render() {
                        tracing::error!("render failed: {:?}", e);
                    }

                    if let Err(e) = tx.send(event) {
                        tracing::error!("send tx failed: {:?}", e);
                    }
                }
                Err(e) => tracing::error!("watch error: {:?}", e),
            })?;

        for path in app.manifest.paths() {
            if !path.exists() {
                continue;
            }
            watcher.watch(&path, notify::RecursiveMode::Recursive)?;
        }

        loop {}
    }

    /// Init project in the given directory.
    pub fn run(&self) -> Result<()> {
        let (tx, rx) = mpsc::channel::<Event>();
        thread::spawn(move || loop {
            if let Err(e) = rx.recv() {
                tracing::error!("watch failed: {:?}", e);
            }
        });

        self.watch(tx)
    }
}
