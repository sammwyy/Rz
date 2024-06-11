use std::path::PathBuf;

use clap::Args;

use crate::clap_utils::ClapCompressionMethod;

#[derive(Debug, Args)]
pub struct WriteOpTarget {
    #[clap(
        name = "sources",
        help = "(Paths) Source files or directories",
        required = true
    )]
    pub src: Vec<PathBuf>,

    #[clap(name = "dest", help = "(Path) Destination file", required = true)]
    pub dest: PathBuf,
}

#[derive(Debug, Args)]
pub struct WriteOpFlags {
    #[arg(short = 'l', long = "level", help = "Compression level to use")]
    pub compression_level: Option<i64>,

    #[arg(short = 'm', long = "method", help = "Compression method to use")]
    pub method: Option<ClapCompressionMethod>,

    #[arg(short = 'u', long = "unix", help = "Set unix permissions (i.e 755)")]
    pub unix_permissions: Option<u32>,
}

#[derive(Debug, Args)]
pub struct ReadOpTarget {
    #[clap(name = "source", help = "(Path) Source file.")]
    pub src: PathBuf,
}

#[derive(Debug, Args)]
pub struct ReadOpFlags {
    #[arg(short = 'p', long = "pick", help = "Pick files to extract")]
    pub pick: Option<Vec<String>>,
}
