use clap::Parser;
#[derive(Debug, Parser)]
#[clap(version)]
pub struct SnatcherArgs {
    /// please provide the path to the alignment file
    pub pafalignment: String,
    /// please provide the path to the fasta file
    pub paf_query_fasta: String,
    /// please provide the path to the reference file
    pub paf_reference_fasta: String,
}
