use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short = "l", long = "length", default_value = "12")]
    length: usize,
}

fn main() {
    let opts = Opts::from_args();
    println!("{:?}", opts);

    let length = opts.length;

    let password = password::generate_password(length);

    println!("Password is: {}", password);
}

