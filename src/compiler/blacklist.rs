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
        let source = match self.file_source.fetch().await {
            Ok(s) => s,
            Err(err) => {
                println!("Could not fetch from {:?}", &self.file_source);
                println!("{}", err);
                println!("Skipping");
                return Vec::new();
            }
        };

        let mut blacklists: Vec<Domain> = Vec::new();
        for line in source.lines() {
            if let Some(bl) = self.parser.parse(line) {
                blacklists.push(bl);
            }
        }

        blacklists
    }
}
