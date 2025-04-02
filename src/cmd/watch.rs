//! command new

use crate::{App, Manifest};
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
    time::Duration,
};

/// Watch command
#[derive(Parser, Debug, Clone)]
pub struct Watch {
    /// The root of the sonata site
    #[clap(default_value = ".")]
    pub dir: PathBuf,

    /// The output directory
    #[clap(short, long, default_value = "out")]
    pub out: PathBuf,
}

impl Watch {
    /// Get the manifest.
    pub fn manifest(&self) -> Result<Manifest> {
        let mut manifest = Manifest::load(&self.dir)?;
        if self.out.is_absolute() || self.out != PathBuf::from("out") {
            manifest.out = self.out.clone();
        }

        Ok(manifest)
    }

    /// Watch the given directory.
    pub fn watch(&self, manifest: Manifest, tx: Sender<Event>) -> Result<()> {
        let mut app: App<'_> = manifest.try_into()?;
        app.livereload();
        app.render()?;

        tracing::info!(
            "watching {} -> {} ...",
            self.dir.display(),
            app.manifest.out.display()
        );

        let paths = app.manifest.paths();
        let mut watcher =
            notify::recommended_watcher(move |res: notify::Result<Event>| match res {
                Ok(event) => {
                    tracing::trace!("event: {:#?}", event);
                    if event.kind == EventKind::Modify(ModifyKind::Metadata(MetadataKind::Any)) {
                        return;
                    }

                    if let Err(e) = app.crender(event.paths.clone()) {
                        tracing::error!("conditional render failed: {:?}", e);
                    }

                    if let Err(e) = tx.send(event) {
                        tracing::error!("send tx failed: {:?}", e);
                    }
                }
                Err(e) => tracing::error!("watch error: {:?}", e),
            })?;

        for path in paths {
            if !path.exists() {
                continue;
            }
            watcher.watch(&path, notify::RecursiveMode::Recursive)?;
        }

        loop {
            thread::sleep(Duration::from_secs(u64::MAX));
        }
    }

    /// Init project in the given directory.
    pub fn run(&self) -> Result<()> {
        let manifest = self.manifest()?;

        let (tx, rx) = mpsc::channel::<Event>();
        thread::spawn(move || loop {
            if let Err(e) = rx.recv() {
                tracing::error!("watch failed: {:?}", e);
            }
        });

        self.watch(manifest, tx)
    }
}
