extern crate getopts;

use getopts::Options;
use std::env;

fn usage(bin: &str, opts: Options) {
	let brief = format!("Usage: {} [options...]", bin);
	print!("{}", opts.usage(&brief));
}

fn version() {
	//show version and build info
}

fn crawl(/*opts: Options*/) {
	println!("ready to crawl");
}

fn main() {
	let argv: Vec<String> = env::args().collect(); 
	let bin = argv[0].clone();
	let mut opts = Options::new();

	opts.optflag("h", "help", "this help");
	opts.optflag("V", "version", "show version and exit");
	opts.optopt("c", "conf", "set configuration file, default fnail.conf", "<file>");

	let m = match opts.parse(&argv[1..]) {
		Ok(a) => { a }
		Err(e) => { panic!(e.to_string()) }
	};

	if m.opt_present("h") {
		usage(&bin, opts);
		return;
	}

	if m.opt_present("V") {
		version();
		return;
	}

	if m.opt_present("c") {
		crawl(/*opts*/);
		return;
	}

	usage(&bin, opts)
}
