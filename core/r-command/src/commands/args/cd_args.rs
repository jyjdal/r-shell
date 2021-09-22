use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Cd", about = "Change directory.")]
pub struct CdArgs {
    #[structopt(default_value = ".")]
    pub dest: String,
}
