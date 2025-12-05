use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "currency-cli", version, about = "Currency converter CLI")]
pub struct Cli {
    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Output format
    #[arg(short = 'O', long, value_enum, default_value_t = OutputFormat::Plain)]
    pub output: OutputFormat,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum OutputFormat {
    Plain,
    Json,
    Csv,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert an amount: <amount> <from> <to>
    Convert {
        amount: f64,
        from: String,
        to: String,
        /// Optional date (YYYY-MM-DD) for historical rates
        #[arg(short, long)]
        date: Option<String>,
    },
    /// Convert amount to multiple targets: <amount> <from> <to1> <to2>...
    Multi {
        amount: f64,
        from: String,
        tos: Vec<String>,
        #[arg(short, long)]
        date: Option<String>,
    },
    /// List available currency codes (from cache or API)
    List {
        #[arg(short, long)]
        refresh: bool,
    },
    /// Update local cache for a base currency
    Update {
        base: String,
    },
}