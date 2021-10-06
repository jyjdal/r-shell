use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Ls", about = "List all child items.")]
pub struct LsArgs {
    #[structopt(short, long)]
    pub all: bool,

    // Display files in reverse order.
    #[structopt(short, long)]
    pub reverse: bool,
}
