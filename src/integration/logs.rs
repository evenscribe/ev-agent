use yaml_rust2::Yaml;

#[derive(Debug)]
pub struct LogsIntegration {
    source: LogSource,
    path: Option<String>,
    service_name: Option<String>,
    start_from: StartFrom,
}

#[derive(Debug)]
pub enum StartFrom {
    Beginning,
    End,
}

#[derive(Debug)]
pub enum LogSource {
    File,
    JournalD,
    DarwinLog,
    // <maybe> TCP,
}

impl LogsIntegration {
    pub fn new(config: &Yaml) -> Self {
        let mut source = None;
        let mut path = None;
        let mut service_name = None;
        let mut start_from = StartFrom::End;

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
                    "start_from" => {
                        if source.is_none() {
                            panic!("`source` field should be defined before `start_from` field");
                        }
                        start_from = match &source {
                            Some(LogSource::File)
                            | Some(LogSource::JournalD)
                            | Some(LogSource::DarwinLog) => match value {
                                "beginning" => StartFrom::Beginning,
                                "end" => StartFrom::End,
                                _ => StartFrom::End,
                            },
                            _ => StartFrom::End,
                        };
                    }
                    _ => panic!("Unknown log integration property: {}", key),
                }
            }
        }

        Self {
            source: source.expect("Log source is required"),
            path,
            service_name,
            start_from,
        }
    }
}
