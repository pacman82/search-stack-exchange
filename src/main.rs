use std::path::PathBuf;

use clap::Parser;

/// Semantic Search on top of stack overflow
#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[clap(subcommand)]
    title: Command
}

#[derive(Parser)]
enum Command {
    /// The question with the title which fits your query best
    Title {
        #[clap(flatten)]
        title_opt: TitleOpt
    }
}


#[derive(Parser)]
struct TitleOpt {
    /// Input Posts.xml for the stack exchange community you want to search
    posts_xml: PathBuf,
}

fn main() {

    let _opt = Cli::parse();


}
