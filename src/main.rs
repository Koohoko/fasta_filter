use clap::{crate_description, crate_name, crate_version, arg, Command};
use fasta_filter::filter;

fn main() {
    // interface and argument matching
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author("Haogao Gu <koohoko@gmail.com>")
        .about(crate_description!())
        .arg(
            arg!(
                -f --file <FILE> "Path of fasta file or use '-' as stdin."
            )
            .required(true)
            .display_order(1)
        )
        .arg(
            arg!(
                -b --base <STRING> "Bases to be accounted for. Examples: \"N,-\". Please note that this is case sensitive."
            )
            .required(false)
            .default_value("N")
            .display_order(2)
        )
        .arg(
            arg!(
                -n --num_base  <NUMBER> "Frequency of specified bases, any sequences with bases count over this threshold will not be print out. Use 0 to skip this step if you only want to use the specified_pos filter."
            )
            .required(true)
            .display_order(3)
        )
        .arg(
            arg!(
                -s --specified_pos_file  <FILE> "Path to a txt file specifying genomic positions of interest, each line should contain one integer specifying nucleotide position. Positions are 1-based rather than 0-based."
            )
            .required(false)
            .display_order(5)
        )
        .arg(
            arg!(
                -m --specified_num_base <NUMBER> "The num_base threshold for the specified positions."
            )
            .required(false)
            .display_order(6)
        )
        .arg(
            arg!(
                -o --out_file <NUMBER> "Path to write to the outfile, if \"-\" will write to stdout."
            )
            .required(false)
            .default_value("-")
            .display_order(8)
        )
        .arg(
            arg!(
                -v --verbose "Add this flap to print parameters to stderr."
            )
            .required(false)
            .display_order(8)
        )
        .get_matches();

    // get arguments
    let check_verbose:bool = match matches.occurrences_of("verbose") {
        0 => false,
        _ => true,
    };
    if check_verbose {eprintln!("{}", "### Job started! ###\n")};

    let mut file_path = "-";
    if let Some(file_path_input) = matches.value_of("file") {
        match file_path_input {
            "-" => {
                file_path = "-";
                if check_verbose {eprintln!("input from stdin")};
            },
            _ => {
                file_path = file_path_input;
                if check_verbose {eprintln!("fasta file: {}", file_path)};
            }
        }
    }

    let mut out_file_path = "-";
    if let Some(file_path_output) = matches.value_of("out_file") {
        match file_path_output {
            "-" => {
                out_file_path = "-";
                if check_verbose {eprintln!("output to stdout")};
            },
            _ => {
                out_file_path = file_path_output;
                if check_verbose {eprintln!("Output file: {}", out_file_path)};
            }
        }
    }

    let mut bases: Vec<char> = matches.value_of("base").unwrap().chars().collect();
    bases.retain(|x| *x != ',');
    if check_verbose {eprintln!("bases: {:?}", bases)};
    
    let mut num_base:usize = 0;
    if let Some(num_base_input) = matches.value_of("num_base"){
        num_base = num_base_input.parse().expect("Please provide a integer.");
        if check_verbose {eprintln!("num_base: {}", num_base)}
    }

    let mut check_specified:bool = false;
    let mut specified_num_base:usize = 0;
    let mut specified_pos_file:&str = "";

    if let Some(input_specified_pos_file) = matches.value_of("specified_pos_file"){
        specified_num_base = matches.value_of("specified_num_base").expect("specified_num_base must be specified if specified_pos_file exists").parse().expect("Please provide a integer.");
        specified_pos_file = input_specified_pos_file;
        check_specified = true
    } 
    if let Some(input_specified_num_base) = matches.value_of("specified_num_base"){
        specified_pos_file = matches.value_of("specified_pos_file").expect("specified_pos_file must be specified if specified_num_base exists");
        specified_num_base = input_specified_num_base.parse().expect("Please provide a integer.");
        check_specified = true
    }

    let bases_u8:Vec<u8> = bases.into_iter().map(|c| c as u8).collect();
    if check_specified {
        if check_verbose {
            eprintln!("specified_pos_file: {}", specified_pos_file);
            eprintln!("specified_num_base: {}", specified_num_base);
        }
        filter::filter_specified(file_path, bases_u8, num_base, specified_pos_file, specified_num_base, out_file_path)
    } else {
        filter::filter(file_path, bases_u8, num_base, out_file_path)
    }    

    if check_verbose {eprintln!("{}", "\n### Job finished! ###")};
}
