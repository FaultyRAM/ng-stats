// The MIT License (MIT)
//
// Copyright (c) 2015 FaultyRAM
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

extern crate docopt;
use docopt::Docopt;
use docopt::Error as DocoptError;

extern crate rustc_serialize;

use std::io::{Write, stderr};

const USAGE: &'static str = "
Usage:
	ng-stats local [-fV] <src-log-dir> <dest-log-dir> <src-html-dir> <dest-html-dir> <db-dir>
	ng-stats world [-V]  <src-log-dir> <dest-url>
	ng-stats (--help | --version)

Options:
	-f, --force    Rebuild the local database
	-V, --verbose  Print verbose information to stdout
	--help         Display this help message and exit
	--version      Display version information and exit
";

#[derive(Debug, RustcDecodable)]
struct Args {
	arg_src_log_dir:   Option<String>,
	arg_dest_log_dir:  Option<String>,
	arg_src_html_dir:  Option<String>,
	arg_dest_html_dir: Option<String>,
	arg_db_dir:        Option<String>,
	arg_dest_url:      Option<String>,
	cmd_local:         bool,
	cmd_world:         bool,
	flag_force:        bool,
	flag_verbose:      bool,
	flag_help:         bool,
	flag_version:      bool,
}

fn main() {
	let parse_result = Docopt::new(USAGE).and_then(|d| {
		d.help(true).version(Some(String::from("ng-stats 0.1.0"))).decode()
	});
	if parse_result.is_err() {
		let e = parse_result.unwrap_err();
		match e {
			DocoptError::Help | DocoptError::Version(_) => {
				println!("{}", e);
			},
			_ => {
				let mut stderr = stderr();
				writeln!(&mut stderr, "{}", e).unwrap();
			},
		}
		return;
	};
	let args: Args = parse_result.unwrap();
	if args.cmd_local {
		unimplemented!()
	}
	if args.cmd_world {
		unimplemented!()
	}
	unreachable!()
}
