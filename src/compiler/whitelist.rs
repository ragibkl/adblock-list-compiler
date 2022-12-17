use crate::compiler::{
    fetch_source::FetchSource,
    parser::{Domain, ParseWhitelist},
};

#[derive(Debug)]
pub struct WhitelistCompiler {
    pub(crate) file_source: FetchSource,
    pub(crate) parser: ParseWhitelist,
}

impl WhitelistCompiler {
    pub async fn load_whitelist(&self) -> Vec<Domain> {
        let source = self.file_source.fetch().await.unwrap();

        let mut blacklists: Vec<Domain> = Vec::new();
        for line in source.lines() {
            if let Some(bl) = self.parser.parse(line) {
                blacklists.push(bl);
            }
        }

        blacklists
    }
}
