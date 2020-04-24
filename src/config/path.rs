use config::Config as C;

#[derive(Debug, Deserialize, Serialize)]
pub struct Path {
    pub images: String,
    pub watch: String,
}

impl Path {
    pub fn default_config() -> Self {
        Path {
            images: "/Volumes/NO NAME/".to_string(),
            watch: "/dev/".to_string(),
        }
    }

    pub fn merge_with_config(&self, config: &mut C, prefix_str: &str) {
        let prefix = |key: &str| -> String { prefix_str.to_string() + key };

        config
            .set_default(&prefix("images"), self.images.to_string())
            .unwrap();

        config
            .set_default(&prefix("watch"), self.watch.to_string())
            .unwrap();
    }
}
