use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    pub name: String,
    #[structopt(short, long)]
    pub greet: Option<String>,
}

#[derive(StructOpt, Debug)]
/// this is help comment!!!
pub enum Opt2 {
    /// greet someone
    Greet { name: String },
    /// say goodbye to someone
    Bye {
        name: String,
        #[structopt(short, long)]
        forever: bool,
    },
}
