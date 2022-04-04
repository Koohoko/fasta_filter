use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
extern crate needletail;
use needletail::{parse_fastx_stdin, parse_fastx_file, FastxReader};


pub fn read_input(file_path:&str) -> Box<dyn FastxReader> {
	let reader = match file_path {
		"-" => parse_fastx_stdin().expect("Not valid stdin"),
		_ => parse_fastx_file(Path::new(file_path)).expect("Not valid input file while parse")
	};
	reader
}

pub fn get_writer(out_file:&str) -> Box<dyn io::Write>{
	let out_writer = match out_file {
		"-" => Box::new(io::stdout()) as Box<dyn io::Write>,
		_ => {
			let path = Path::new(out_file);
			Box::new(File::create(&path).unwrap()) as Box<dyn io::Write>
		}
	};
	out_writer
}

pub fn read_specified_pos(specified_pos_file:&str) -> Vec<usize> {
	let lines = read_lines(specified_pos_file).expect("Error when reading specified_pos_file");
	let mut pos_vec=Vec::new();
	// Consumes the iterator, returns an (Optional) String
	for line in lines {
		if let Ok(pos) = line {
			if pos.trim().len() == 0 {
				continue;
			}
			let pos_num:usize = match pos.parse::<usize>() {
				Ok(num) => num-1,
				Err(e) => panic!("{} is not a number in specified_pos_file, {}", pos, e),
			};
			pos_vec.push(pos_num);
		}
	}
	return pos_vec

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}