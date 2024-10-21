mod tailer;

use crate::event::Event;
use std::{io::SeekFrom, sync::Arc};
use tailer::{FileTailer, Tailer};
use tokio::sync::mpsc;
use yaml_rust2::Yaml;

#[derive(Debug)]
pub struct LogsIntegration {
    source: LogSource,
    service_name: String, // for journald and darwinlog
    tailer: Box<dyn Tailer>,
}

#[derive(Debug)]
pub enum LogSource {
    File,
    JournalD,
    DarwinLog,
}

impl LogsIntegration {
    pub async fn new(
        config: &Yaml,
        service_transmitter: mpsc::Sender<Arc<Event>>,
        owner: &str,
    ) -> Self {
        let mut source = None;
        let mut path = None;
        let mut service_name = None;
        let mut seek_from = SeekFrom::End(0);

        let config = config
            .as_vec()
            .expect("Logs integration properties should be a list.");

        for c in config {
            let config_hash = c
                .as_hash()
                .expect("Each log integration property should be a key-value pair");
            for (key, value) in config_hash.iter() {
                let key = key
                    .as_str()
                    .expect("Log integration property key should be a string");
                let value = value
                    .as_str()
                    .expect("Log integration property value should be a string");
                match key {
                    "source" => {
                        source = match value {
                            "file" => Some(LogSource::File),
                            "journald" => Some(LogSource::JournalD),
                            "darwinlog" => Some(LogSource::DarwinLog),
                            _ => panic!("Unknown log source: {}", value),
                        }
                    }
                    "path" => {
                        if source.is_none() {
                            panic!("`source` field should be defined before `path` field");
                        }
                        path = match &source {
                            Some(LogSource::File) => Some(value.to_owned()),
                            _ => panic!("`path` field is not supported for this source"),
                        };
                    }
                    "service_name" => {
                        if source.is_none() {
                            panic!("`source` field should be defined before `service_name` field");
                        }
                        service_name = match &source {
                            Some(LogSource::JournalD) | Some(LogSource::DarwinLog) => {
                                Some(value.to_owned())
                            }
                            _ => panic!("`service_name` field is not supported for this source"),
                        };
                    }
                    "seek_from" => {
                        if source.is_none() {
                            panic!("`source` field should be defined before `seek_from` field");
                        }
                        seek_from = match &source {
                            Some(LogSource::File)
                            | Some(LogSource::JournalD)
                            | Some(LogSource::DarwinLog) => match value {
                                "beginning" => SeekFrom::Start(0),
                                "end" => SeekFrom::End(0),
                                _ => SeekFrom::End(0),
                            },
                            _ => SeekFrom::End(0),
                        };
                    }
                    _ => panic!("Unknown log integration property: {}", key),
                }
            }
        }

        let tailer = match &source {
            Some(LogSource::File) => Box::new(
                FileTailer::new(
                    path.unwrap(),
                    owner.to_string(),
                    seek_from,
                    service_transmitter,
                )
                .await,
            ),
            _ => unimplemented!(),
        };

        Self {
            source: source.expect("Log source is required"),
            service_name: service_name.unwrap_or_else(|| owner.to_string()),
            tailer,
        }
    }

    pub async fn run(&mut self) {
        self.tailer.tail().await;
    }
}
