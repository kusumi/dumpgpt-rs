mod gpt;
mod subr;
mod uuid;

const VERSION: [i32; 3] = [0, 1, 8];

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default)]
struct Opt {
    verbose: bool,
    symbol: bool,
    noalt: bool,
}

fn get_version_string() -> String {
    format!("{}.{}.{}", VERSION[0], VERSION[1], VERSION[2])
}

fn print_version() {
    println!("{}", get_version_string());
}

fn usage(progname: &str, opts: &getopts::Options) {
    print!(
        "{}",
        opts.usage(&format!("usage: {progname} [<options>] <paths>"))
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let progname = &args[0];

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
        Err(e) => {
            println!("{e}");
            usage(progname, &opts);
            std::process::exit(1);
        }
    };
    if matches.opt_present("v") {
        print_version();
        std::process::exit(1);
    }
    if matches.opt_present("h") {
        usage(progname, &opts);
        std::process::exit(1);
    }

    let mut opt = Opt {
        ..Default::default()
    };
    opt.verbose = matches.opt_present("verbose");
    opt.symbol = matches.opt_present("symbol");
    opt.noalt = matches.opt_present("noalt");

    if opt.verbose {
        print_version();
    }

    if matches.free.is_empty() {
        usage(progname, &opts);
        std::process::exit(1);
    }

    let device = &matches.free[0];
    println!("{device}");
    println!();

    let mut fp = match std::fs::File::open(device) {
        Ok(v) => v,
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }
    };
    if let Err(e) = gpt::dump_gpt(&mut fp, &opt) {
        panic!("{e}");
    }
}
