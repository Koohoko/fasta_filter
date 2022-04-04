// extern crate needletail;
use needletail::{Sequence};
use std::{str, borrow::Cow};
use crate::io::{get_writer, read_input, read_specified_pos};

pub fn filter(file_path:&str, bases_u8:Vec<u8>, num_base:u32, out_file:&str) {
	let mut reader = read_input(file_path);
	let mut writer = get_writer(out_file);
	// print!("{:?}", bases_u8);

	while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
		let seq = seqrec.strip_returns();
		let n_count = count_n_base(&bases_u8, &seq);

		if n_count <= num_base{
			seqrec.write(&mut writer, None).expect("write error");
		}
    }
}

pub fn filter_specified(file_path:&str, bases_u8:Vec<u8>, num_base:u32, specified_pos_file:&str, specified_num_base:u32, out_file:&str) {
	let mut reader = read_input(file_path);
	let mut writer = get_writer(out_file);
	let mut_pos = read_specified_pos(specified_pos_file);

	while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
		let seq = seqrec.strip_returns();
		// TODO
		if num_base == 0 {
			let n_count_specified = count_specified(&bases_u8, &seq, &mut_pos);
			if n_count_specified <= specified_num_base{
				seqrec.write(&mut writer, None).expect("write error");
			}
		} else {
			let n_count = count_n_base(&bases_u8, &seq);
			if n_count <= num_base{
				let n_count_specified = count_specified(&bases_u8, &seq, &mut_pos);
				if n_count_specified <= specified_num_base{
					seqrec.write(&mut writer, None).expect("write error");
				}
			}
		}
    }
}

fn count_n_base(bases_u8:&Vec<u8>, seq:&Cow<[u8]>) -> u32{
	let mut n_count:u32 = 0;
	for char_u8 in seq.as_ref(){
		for base_u8 in bases_u8{
			if char_u8 == base_u8 {
				n_count += 1
			}
		}
	}
	n_count
}

fn count_specified(bases_u8:&Vec<u8>, seq:&Cow<[u8]>, mut_pos:&Vec<usize>) -> u32{
	let mut n_count:u32 = 0;
	for pos in mut_pos {
		for base_u8 in bases_u8{
			if seq.as_ref()[*pos] == *base_u8 {
				n_count += 1
			}
		}
	}
	n_count
}


