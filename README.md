## fasta_filter

This little tool accept a fasta file (or from stdin), and out put a filtered fasta file. It is (relatively) fast, see [comparison](#benchmark) between a simple python implementation.

### Usage:
```
fasta_filter 0.1.3
Haogao Gu <koohoko@gmail.com>
A tool for filtering fasta sequences with threshold of specific bases (e.g. 'N'), written in Rust.

USAGE:
    fasta_filter [OPTIONS] --file <FILE> --num_base <NUMBER>

OPTIONS:
    -f, --file <FILE>
            Path of fasta file or use '-' as stdin.

    -b, --base <STRING>
            Bases to be accounted for. Examples: "N,-". Please note that this is case sensitive.
            [default: N]

    -n, --num_base <NUMBER>
            Frequency of specified bases, any sequences with bases count over this threshold will
            not be print out. Use 0 to skip this step if you only want to use the specified_pos
            filter.

    -s, --specified_pos_file <FILE>
            Path to a txt file specifying genomic positions of interest, each line should contain
            one integer specifying nucleotide position. Positions are 1-based rather than 0-based.

    -m, --specified_num_base <NUMBER>
            The num_base threshold for the specified positions.

    -o, --out_file <NUMBER>
            Path to write to the outfile, if "-" will write to stdout. [default: -]

    -v, --verbose
            Add this flap to print parameters to stderr.

    -h, --help
            Print help information

    -V, --version
            Print version information
```

### Installation
#### Executable
Directly download executables from [Releases](https://github.com/Koohoko/fasta_filter/releases).
#### Install from source
1. Install Rust from [here](https://www.rust-lang.org/tools/install).
2. Download source code by `git clone https://github.com/Koohoko/fasta_filter.git`.
3. Install with `cargo install --path fasta_filter`.
4. You are ready to go.

### Examples
**Example input:**
```
>seq1_8N_5del
NNNAAAAAAAAAAAAACCCCCCCCCCCTTTTTTTTGGGGGGGGGGGGGGGGAAACCC-----AAAAAANNNNNTTTTT
>seq2_20N_10del
NNNAAAAAAAAA-----CCCCCCCCCTTTTTTTTGGGGGGGNNNNNNNGGAAACCC-----AAAAAANNNNNNNNNNT
>seq1_5N_5del
TTTAAAAAAAAAAAAACCCCCCCCCCCTTTTTTTTGGGGGGGGGGGGGGGGAAACCC-----AAAAAANNNNNTTTTT
```
**Example usage:**
- Drop the sequences with > 5 "N" bases:
```sh
✗ fasta_filter -b N -n 5 -f data/small.fasta
>seq3_5N_5del
TTTAAAAAAAAAAAAACCCCCCCCCCCTTTTTTTTGGGGGGGGGGGGGGGGAAACCC-----AAAAAANNNNNTTTTT
```
- Drop the sequences with > 20 "N"+"-" bases:
```sh
✗ fasta_filter -b N,- -n 20 -f data/small.fasta 
>seq1_8N_5del
NNNAAAAAAAAAAAAACCCCCCCCCCCTTTTTTTTGGGGGGGGGGGGGGGGAAACCC-----AAAAAANNNNNTTTTT
>seq3_5N_5del
TTTAAAAAAAAAAAAACCCCCCCCCCCTTTTTTTTGGGGGGGGGGGGGGGGAAACCC-----AAAAAANNNNNTTTTT
```
- Drop the sequences with > 2 "N bases within specified positions (positions are specified in a [txt file](data/mut_pos.txt)). Here we use "-n 0" to skip the full genome filter:
```sh
✗ fasta_filter -b N -n 0 -f data/small.fasta -s ./data/mut_pos.txt -m 2
>seq2_20N_10del
NNAAAAAAAAAA-----CCCCCCCCCTTTTTTTTGGGGGGGNNNNNNNGGAAACCC-----AAAAAANNNNNNNNNNT
>seq3_5N_5del
TTTAAAAAAAAAAAAACCCCCCCCCCCTTTTTTTTGGGGGGGGGGGGGGGGAAACCC-----AAAAAANNNNNTTTTT
```
- filtering both specified positions and the full genome with different threshold:
```sh
✗ fasta_filter -b N -n 10 -f data/small.fasta -s ./data/mut_pos.txt -m 2
>seq3_5N_5del
TTTAAAAAAAAAAAAACCCCCCCCCCCTTTTTTTTGGGGGGGGGGGGGGGGAAACCC-----AAAAAANNNNNTTTTT
```
- Input compressed files and output to a regular fasta file, showing verbose info:
```sh
✗ fasta_filter -b N -n 10 -f data/small.fasta.xz -s ./data/mut_pos.txt -m 2 -v -o ./data/test_output.fasta
### Job started! ###

fasta file: data/small.fasta.xz
Output file: ./data/test_output.fasta
bases: ['N']
num_base: 10
allow_iupac: true
specified_pos_file: ./data/mut_pos.txt
specified_num_base: 2

### Job finished! ###

✗ cat ./data/test_output.fasta 
>seq1_8N_5del
TTTAAAAAAAAAAAAACCCCCCCCCCCTTTTTTTTGGGGGGGGGGGGGGGGAAACCC-----AAAAAANNNNNTTTTT
```

### Benchmark
- Runing on plain fasta file containing SASR-CoV-2 sequences (it is 1.5GB in file size, and contains 50,000 sequences (length of each sequence ~ 29900)). fasta_filter used ~2s and the [python3](./python/sequence_cleaner.py) one used ~12s on my poor computer (Intel NUC8i5beh). Details can be found here ([Rust](./data/benchmark_rust.log), [Python]("./data/benchmark_python.log")).

- Using filter_fasta with double filters for a big fasta file (302GB in plain text, multiple sequence alignment of SARS-CoV-2 downloaded from GISAID). IO seems to be the bottleneck.
```
✗ time -hl ./target/release/fasta_filter -f /Volumes/SSD_480G/Downloads/msa_2022-04-04/2022-04-04_unmasked.fa -b n,- -n 3000 -m 10 -s ./data/BA1_BA2_pos.txt -o /Users/koohoko/Downloads/2022-04-04_unmasked_filtered.fasta 
	10m14.25s real		2m47.21s user		4m53.15s sys
             1507328  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                 534  page reclaims
                   0  page faults
                   0  swaps
                   0  block input operations
                  10  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
               26996  voluntary context switches
             1138031  involuntary context switches
       2356529992444  instructions retired
       1433719284879  cycles elapsed
              667648  peak memory footprint
```

### TODO / PLANS
* [x] Test pipe streams. Stdin and Stdout work as expected.
* [x] Test zip files. gz and xz inputs are also supported.
* [x] Benchmark against python implementation.
* [ ] Add installation instruction.
* [ ] Work in multithread mode?


---

