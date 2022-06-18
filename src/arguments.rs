use clap::{clap_derive::ArgEnum, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct RuuhidOps {
    #[clap(subcommand)]
    pub subcommand: Option<RuuhidSubOpts>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum RuuhidSubOpts {
    Generate {
        subcommand: Option<RuuhidGenVersionOpts>,
    },
}

#[derive(Clone, Debug, Subcommand)]
pub enum RuuhidGenVersionOpts {
    Nil(NilOpts),
    Mac,
    Dce,
    Md5,
    Random,
    Sha1,
}

#[derive(Clone, Debug, Parser)]
pub struct NilOpts {
    pub format: FormatOps,
}

#[derive(Debug, Clone, ArgEnum)]
pub enum FormatOps {
    Bytes,
    Guid,
    Hyphenated,
    Simple,
    Urn,
}

impl std::str::FromStr for FormatOps {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!();
    }
}

impl std::str::FromStr for RuuhidGenVersionOpts {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
