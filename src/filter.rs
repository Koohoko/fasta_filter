extern crate needletail;
use needletail::{parse_fastx_stdin, parse_fastx_file, Sequence, FastxReader};
use std::str;
use std::{io, io::Write};
use std::fs::File;
use std::path::Path;
use std::borrow::Cow;

pub fn filter(file_path:&str, check_ipuac:bool, bases:Vec<char>, num_base:i32, out_file:&str) {
	let mut reader = read_input(file_path);
	let mut writer = get_writer(out_file);

	while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");	
		let mut n_count = 0;
		let mut seq:Cow<[u8]>;
		if check_ipuac {
			seq = seqrec.normalize(true);
			n_count = count_n_base(&bases, seq);
		} else{
			seq = seqrec.normalize(false);
			n_count = count_n_base(&bases, seq);
		}
		// TODO
		if n_count <= num_base{
			// println!("ID: {:?}", str::from_utf8(seqrec.id()).expect("can't get ID from utf8"));
			// println!("num of N: {}", n_count);
			seqrec.write(&mut writer, None).expect("write error");
		}
    }
}

pub fn filter_specified(file_path:&str, check_ipuac:bool, bases:Vec<char>, num_base:i32, specified_pos_file:&str, specified_num_base:i32, out_file:&str) {
	let mut reader = read_input(file_path);
	let writer = get_writer(out_file);

	if num_base == 0{
			
	} else{
		
	}

}

fn read_input(file_path:&str) -> Box<dyn FastxReader> {
	if file_path=="-" {
		return parse_fastx_stdin().expect("Not valid stdin")
	} else {
		return parse_fastx_file(Path::new(file_path)).expect("Not valid input file while parse")
	}	
}

fn count_n_base(bases:&Vec<char>, seq:Cow<[u8]>) -> i32{
	let mut n_count = 0;

	match str::from_utf8(&seq) {
		// Ok(v) => n_count+=1,
		Ok(v) => {
			for char in v.chars(){
				for base in bases{
					if char == *base {
						n_count += 1
					}
				}
			}
		},
		Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
	}
	n_count
}

fn get_writer(out_file:&str) -> Box<dyn Write>{
	let out_writer = match out_file {
		"-" => Box::new(io::stdout()) as Box<dyn Write>,
		_ => {
			let path = Path::new(out_file);
			Box::new(File::create(&path).unwrap()) as Box<dyn Write>
		}
	};
	out_writer
}
