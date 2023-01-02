mod gpt;
mod subr;
mod uuid;

const VERSION: [i32; 3] = [0, 1, 0];

#[derive(Debug, Default)]
struct UserOption {
    verbose: bool,
    symbol: bool,
    noalt: bool,
}

#[derive(Debug, Default)]
pub struct UserData {
    opt: UserOption,
}

fn get_version_string() -> String {
    format!("{}.{}.{}", VERSION[0], VERSION[1], VERSION[2])
}

fn print_version() {
    println!("{}", get_version_string());
}

fn usage(progname: &str, opts: getopts::Options) {
    let brief = format!("usage: {} [<options>] <paths>", progname);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let progname = args[0].clone();

    subr::assert_ds();

    if !subr::is_le() {
        println!("big-endian arch unsupported");
        std::process::exit(1);
    }

    let mut opts = getopts::Options::new();
    opts.optflag("", "verbose", "Enable verbose print");
    opts.optflag("", "symbol", "Print symbol name if possible");
    opts.optflag("", "noalt", "Do not dump secondary header and entries");
    opts.optflag("v", "version", "Print version and exit");
    opts.optflag("h", "help", "Print usage and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string()),
    };
    if matches.opt_present("v") {
        print_version();
        std::process::exit(1);
    }
    if matches.opt_present("h") {
        usage(&progname, opts);
        std::process::exit(1);
    }

    let mut dat = UserData {
        ..Default::default()
    };
    dat.opt.verbose = matches.opt_present("verbose");
    dat.opt.symbol = matches.opt_present("symbol");
    dat.opt.noalt = matches.opt_present("noalt");

    if dat.opt.verbose {
        print_version();
    }

    if matches.free.is_empty() {
        usage(&progname, opts);
        std::process::exit(1);
    }

    let args = matches.free;
    let device = &args[0];
    println!("{}", device);
    println!();

    let mut fp = match std::fs::File::open(device) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    match gpt::dump_gpt(&mut fp, &dat) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
