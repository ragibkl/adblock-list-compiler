use url::Url;

use crate::service::fetch_source::{FetchHTTP, FetchSource};
use crate::service::parser::{BlacklistParser, HostsParser};
use crate::service::compiler::BlacklistCompiler;

pub async fn compile() {
    let bl = BlacklistCompiler {
        file_source: FetchSource::HTTP(FetchHTTP {
            url: Url::parse("https://sebsauvage.net/hosts/hosts").unwrap(),
        }),
        parser: BlacklistParser::Hosts(HostsParser {}),
    };

    let s = bl.load_blacklist().await;

    println!("{:#?}", s);
}
