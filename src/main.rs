mod opt;
use opt::Opt;
use opt::Opt2;
use structopt::StructOpt;

fn main() {
    // let opt = Opt::from_args();
    // match opt.greet {
    //     Some(greet) => println!("{}, {}", greet, opt.name),
    //     None => println!("{}", opt.name),
    // }

    match Opt2::from_args() {
        Opt2::Greet { name } => println!("{}さん、こんにちは", name),
        Opt2::Bye { name, forever } => {
            if forever {
                println!("{}さん、永遠にさようなら", name)
            } else {
                print!("{}さん、一旦さようなら", name)
            }
        }
    }
}
