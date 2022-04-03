use clap::{arg, Command};
use fasta_filter::filter;

fn main() {
    let matches = Command::new("fasta_filter")
        .version("0.1.0")
        .author("Haogao Gu <koohoko@gmail.com>")
        .about("Filtering fasta file with base frequencies")
        .arg(
            arg!(
                -f --file <FILE> "Path of fasta file or use '-' as stdin."
            )
            .required(true)
            .display_order(1)
        )
        .arg(
            arg!(
                -b --base <STRING> "Bases to be accounted for. Examples: \"N,-\" [Default: 'N']. Please note that everything other than \"AGCTN-\" is considered a N."
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
                --allow_iupac <BOOLEAN> "Should IPUAC bases be allowed? (true or false). If true, IPUAC bases will not be converted to Ns. [Default: true]"
            )
            .required(false)
            .default_value("true")
            .display_order(4)
        )
        .arg(
            arg!(
                -s --specified_pos_file  <FILE> "Path to a txt file specifying genomic positions of interest, each line should contain one integer specifying nucleotide position."
            )
            .required(false)
            .display_order(5)
        )
        .arg(
            arg!(
                --specified_num_base <NUMBER> "The num_base threshold for the specified positions."
            )
            .required(false)
            .display_order(6)
        )
        .arg(
            arg!(
                -o --out_file <NUMBER> "Path to write to the outfile, if \"-\" will write to stdout. [Default: -]"
            )
            .required(false)
            .default_value("-")
            .display_order(8)
        )
        .get_matches();


    eprintln!("{}", "### Job started! ###\n");
    let mut file_path = "-";
    if let Some(file_path_input) = matches.value_of("file") {
        match file_path_input {
            "-" => {
                file_path = "-";
                eprintln!("input from stdin");
            },
            _ => {
                file_path = file_path_input;
                eprintln!("fasta file: {}", file_path);
            }
        }
    }

    let mut out_file_path = "-";
    if let Some(file_path_output) = matches.value_of("out_file") {
        match file_path_output {
            "-" => {
                out_file_path = "-";
                eprintln!("output to stdout");
            },
            _ => {
                out_file_path = file_path_output;
                eprintln!("Output file: {}", out_file_path);
            }
        }
    }

    // if from_stdin {
    //     println!("From stdin: {}", from_stdin);
    // }

    let mut bases: Vec<char> = matches.value_of("base").unwrap().chars().collect();
    bases.retain(|x| *x != ',');
    eprintln!("bases: {:?}", bases);
    
    let mut num_base = 0;
    if let Some(num_base_input) = matches.value_of("num_base"){
        num_base = num_base_input.parse().expect("Please provide a integer.");
        eprintln!("num_base: {}", num_base)
    }

    let mut check_specified:bool = false;
    let mut specified_num_base:i32 = 0;
    let mut specified_pos_file:&str = "";
    let check_ipuac:bool = match matches.value_of("allow_iupac").unwrap() {
        "true" => true,
        "false" => false,
        _ => false
    };
    eprintln!("allow_iupac: {}", check_ipuac);

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

    if check_specified {
        eprintln!("specified_pos_file: {}", specified_pos_file);
        eprintln!("specified_num_base: {}", specified_num_base);
        filter::filter_specified(file_path, check_ipuac, bases, num_base, specified_pos_file, specified_num_base, out_file_path)
    } else {
        filter::filter(file_path, check_ipuac, bases, num_base, out_file_path)
    }    


    eprintln!("{}", "\n### Job finished! ###");
}
