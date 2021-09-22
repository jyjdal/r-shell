use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Cat", about = "Read a file and display its content.")]
pub struct CatArgs {
    pub file_name: String,
}
