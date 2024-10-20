use crate::integration::Integration;
use yaml_rust2::Yaml;

#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub integrations: Vec<Integration>,
}

impl Service {
    pub fn new(service: &Yaml) -> Self {
        let service = service.as_hash().expect("Service should be a hash");
        let (key, value) = service
            .iter()
            .next()
            .expect("Service should have a single key-value pair");
        let name = key.as_str().expect("Service name should be a string");

        let mut integrations = vec![];

        let integrations_yaml = value
            .as_vec().
            expect(&format!("Integrations array under {} has not be formatted properly in config.\nHint: It should be an array of objects", name));

        for integration in integrations_yaml {
            integrations.push(Integration::new(&integration))
        }

        Self {
            name: name.to_string(),
            integrations,
        }
    }
}
