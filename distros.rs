// A Command-line tool to play Marco Polo
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift", about = "hello world")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}   

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Greet {
        #[clap(short, long)]
        name: String,
    },
}
