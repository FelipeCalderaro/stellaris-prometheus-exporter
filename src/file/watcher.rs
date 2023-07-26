use std::path::Path;

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use log::{debug, error, info, trace, warn};
use notify::{Config, Error, Event, RecommendedWatcher, RecursiveMode, Watcher};

use crate::{
    file::save_handler::{convert_to_pretty_str, parse_save_file, save_json_to_file},
    singletons::singletons::set_game_data,
};

pub fn spawn_file_watcher(path: String) {
    // let profile = std::env::var("USERPROFILE").unwrap();
    info!("Starting file watcher");
    tokio::spawn(async {
        if let Err(e) = async_watch(path).await {
            error!("{:?}", e)
        }
    });
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

pub async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    // let (mut watcher, mut rx): (RecommendedWatcher, Event) = async_watcher()?;
    let (mut watcher, mut rx): (RecommendedWatcher, Receiver<Result<Event, Error>>) =
        async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                trace!("New event: {:?}", event);
                match event.kind {
                    notify::event::EventKind::Create(_) => {
                        trace!("New entry created in the save directory ");
                        if event.paths.len() > 0 {
                            let _path = event.paths.get(0);
                            if let Some(created_path) = _path {
                                trace!("Current Creation: {:?}", created_path);
                                if created_path.is_file()
                                    && created_path.extension().unwrap() == "sav"
                                {
                                    info!("Parsing new save file");
                                    if let Some(path) = created_path.as_path().to_str() {
                                        let content = match parse_save_file(path) {
                                            Ok(c) => c,
                                            Err(_) => return Ok(()),
                                        };
                                        debug!("Parsed Game ID: {}", &content.game_id);
                                        debug!(
                                            "Parsed content length: {:?}",
                                            &content.gamestate.len()
                                        );
                                        std::env::set_var("STELLARIS_FILENAME", content.filename);
                                        std::env::set_var("STELLARIS_GAMEID", content.game_id);

                                        if let Ok(pretty) =
                                            convert_to_pretty_str(*content.gamestate)
                                        {
                                            let _ = set_game_data(pretty.clone());
                                            let _ = save_json_to_file(&Box::new(pretty));
                                        }

                                        info!("Save file parsed");
                                    } else {
                                        warn!("The entry has no Path");
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
