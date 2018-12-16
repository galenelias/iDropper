extern crate clap;
extern crate itertools;
extern crate matroska;

use clap::{Arg,App};
// use std::io::{self, BufRead};
use std::fs::File;
use matroska::Matroska;

fn main() {
	let matches = App::new("iDropper")
		.author("Galen Elias, gelias@gmail.com")
		.version("0.1.0")
		.about("Tool for data moshing video files")
		.arg(
			Arg::with_name("file")
				.short("f")
				.takes_value(true)
				.help("Uses a file instead of reading from standard in"))
		.after_help("Longer explaination to appear after the options when \
					displaying the help information from --help or -h")
		.get_matches();

	if matches.is_present("file") {
		let f = File::open(matches.value_of("file").unwrap()).unwrap();
		// let file = BufReader::new(&f);

		// let f = File::open("filename.mkv").unwrap();
		let matroska = Matroska::open(f).unwrap();
		println!("title : {:?}", matroska.info.title);
		println!("duration : {:?}", matroska.info.duration);

		println!("Tracks: {}", matroska.tracks.len());
		println!("Attachments: {}", matroska.attachments.len());
		println!("Chapters: {}", matroska.chapters.len());

		for track in matroska.tracks {
			println!("---Track {}--", track.number);
			println!("{:?}", track);
		}
	}

}
