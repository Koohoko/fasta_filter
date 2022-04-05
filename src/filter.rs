// extern crate needletail;
use needletail::Sequence;
use std::{str, borrow::Cow};
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::fs::File;
use crate::reader::{read_input, read_specified_pos};

pub fn filter(file_path:&str, bases_u8:Vec<u8>, num_base:usize, out_file:&str) {
	let mut reader = read_input(file_path);
	let stdout = io::stdout();
	let mut writer: BufWriter<Box<dyn Write>> = match out_file {
		"-" => {
    		let lock = stdout.lock();
			BufWriter::new(Box::new(lock))
		},
		_ => {
			let path = Path::new(out_file);
        	BufWriter::new(Box::new(File::create(path).unwrap()))
		},
	};
	
	while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
		let seq = seqrec.strip_returns();
		let n_count = count_n_base(&bases_u8, &seq, &num_base); //TODO: return a boolean will reduce one check in the next line
		
		if n_count <= num_base{
			writer.write_all(b">").unwrap();
			writer.write_all(seqrec.id()).unwrap();
			writer.write_all(b"\n").unwrap();
			writer.write_all(seqrec.raw_seq()).unwrap();
			writer.write_all(b"\n").unwrap();
		}
    }
}

pub fn filter_specified(file_path:&str, bases_u8:Vec<u8>, num_base:usize, specified_pos_file:&str, specified_num_base:usize, out_file:&str) {
	let mut reader = read_input(file_path);
	let stdout = io::stdout();
	let mut writer: BufWriter<Box<dyn Write>> = match out_file {
		"-" => {
    		let lock = stdout.lock();
			BufWriter::new(Box::new(lock))
		},
		_ => {
			let path = Path::new(out_file);
        	BufWriter::new(Box::new(File::create(path).unwrap()))
		},
	};
	let mut_pos = read_specified_pos(specified_pos_file);

	while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
		let seq = seqrec.strip_returns();
		// TODO
		if num_base == 0 {
			let n_count_specified = count_specified(&bases_u8, &seq, &mut_pos, &specified_num_base);
			if n_count_specified <= specified_num_base{
				writer.write_all(b">").unwrap();
				writer.write_all(seqrec.id()).unwrap();
				writer.write_all(b"\n").unwrap();
				writer.write_all(seqrec.raw_seq()).unwrap();
				writer.write_all(b"\n").unwrap();
			}
		} else {
			let n_count_specified = count_specified(&bases_u8, &seq, &mut_pos, &specified_num_base);
			if n_count_specified <= specified_num_base{
				let n_count = count_n_base(&bases_u8, &seq, &num_base);
				if n_count <= num_base{
					writer.write_all(b">").unwrap();
					writer.write_all(seqrec.id()).unwrap();
					writer.write_all(b"\n").unwrap();
					writer.write_all(seqrec.raw_seq()).unwrap();
					writer.write_all(b"\n").unwrap();
				}
			}
		}
    }
}

fn count_n_base(bases_u8:&Vec<u8>, seq:&Cow<[u8]>, num_base:&usize) -> usize{
	let mut n_count:usize = 0;
	for base_u8 in bases_u8{
		n_count += bytecount::count(seq.as_ref(), *base_u8);
		if n_count > *num_base {break;}
	}
	n_count
}

fn count_specified(bases_u8:&Vec<u8>, seq:&Cow<[u8]>, mut_pos:&Vec<usize>, specified_num_base:&usize) -> usize{
	let mut n_count:usize = 0;
	for base_u8 in bases_u8{
		for pos in mut_pos {
			if seq.as_ref()[*pos] == *base_u8 {
				n_count += 1
			}
		}
		if n_count > *specified_num_base {break;}
	}
	n_count
}


