use crate::compiler::{
    fetch_source::FetchSource,
    parser::{Domain, ParseBlacklist},
};

#[derive(Debug)]
pub struct BlacklistCompiler {
    pub(crate) file_source: FetchSource,
    pub(crate) parser: ParseBlacklist,
}

impl BlacklistCompiler {
    pub async fn load_blacklist(&self) -> Vec<Domain> {
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
