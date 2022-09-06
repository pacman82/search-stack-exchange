use clap::Parser;

#[derive(Parser)]
#[clap(version)]
struct Cli {

}

fn main() {

    let _opt = Cli::parse();
}
