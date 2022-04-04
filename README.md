### fasta_filter

This little tool accept a fasta file (or from stdin), and out put a filtered fasta file. It is (relatively) fast, see [comparison](#benchmark) between a simple python implementation.

#### Usage:
```
fasta_filter 0.1.1
Haogao Gu <koohoko@gmail.com>
A tool for filtering fasta sequences with threshold of specific bases (e.g. 'N'), written in Rust.

USAGE:
    fasta_filter [OPTIONS] --file <FILE> --num_base <NUMBER>

OPTIONS:
    -f, --file <FILE>
            Path of fasta file or use '-' as stdin.

    -b, --base <STRING>
            Bases to be accounted for. Examples: "N,-" [Default: 'N']. Please note that everything
            other than "AGCTN-" is considered a N. [default: N]

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
            Path to write to the outfile, if "-" will write to stdout. [Default: -] [default: -]

    -v, --verbose
            Add this flap to print parameters to stderr.

    -h, --help
            Print help information

    -V, --version
            Print version information
```

#### Examples
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
✗ ./fasta_filter -b N -n 5 -f data/small.fasta   
>seq3_5N_5del
TTTAAAAAAAAAAAAACCCCCCCCCCCTTTTTTTTGGGGGGGGGGGGGGGGAAACCC-----AAAAAANNNNNTTTTT
```
- Drop the sequences with > 20 "N"+"-" bases:
```sh
✗ ./fasta_filter -b N,- -n 20 -f data/small.fasta 
>seq1_8N_5del
NNNAAAAAAAAAAAAACCCCCCCCCCCTTTTTTTTGGGGGGGGGGGGGGGGAAACCC-----AAAAAANNNNNTTTTT
>seq3_5N_5del
TTTAAAAAAAAAAAAACCCCCCCCCCCTTTTTTTTGGGGGGGGGGGGGGGGAAACCC-----AAAAAANNNNNTTTTT
```
- Drop the sequences with > 2 "N bases within specified positions (positions are specified in a [txt file](data/mut_pos.txt)). Here we use "-n 0" to skip the full genome filter:
```sh
✗ ./fasta_filter -b N -n 0 -f data/small.fasta -s ./data/mut_pos.txt -m 2
>seq2_20N_10del
NNAAAAAAAAAA-----CCCCCCCCCTTTTTTTTGGGGGGGNNNNNNNGGAAACCC-----AAAAAANNNNNNNNNNT
>seq3_5N_5del
TTTAAAAAAAAAAAAACCCCCCCCCCCTTTTTTTTGGGGGGGGGGGGGGGGAAACCC-----AAAAAANNNNNTTTTT
```
- filtering both specified positions and the full genome with different threshold:
```sh
✗ ./fasta_filter -b N -n 10 -f data/small.fasta -s ./data/mut_pos.txt -m 2
>seq3_5N_5del
TTTAAAAAAAAAAAAACCCCCCCCCCCTTTTTTTTGGGGGGGGGGGGGGGGAAACCC-----AAAAAANNNNNTTTTT
```
- Input compressed files and output to a regular fasta file, showing verbose info:
```sh
✗ ./fasta_filter -b N -n 10 -f data/small.fasta.xz -s ./data/mut_pos.txt -m 2 -v -o ./data/test_output.fasta
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

#### Benchmark
Runing on plain fasta file containing SASR-CoV-2 sequences (it is 1.5GB in file size, and contains 50,000 sequences (length of each sequence ~ 29900)). The rust implementation can be twice faster than the python3 one. I believe there is some space to further improve the performance, and I will try update it in later version.

- For filter_fasta
```sh
✗ time -hl ./target/release/fasta_filter -f /Users/koohoko/Downloads/test.fasta -b N,- -n 4500 -m 10 -s ./data/BA1_BA2_pos.txt > /Users/koohoko/Downloads/benchmark_rust.fasta 2> ./data/benchmark_rust.log
	5.33s real		4.18s user		1.06s sys
             1363968  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                 459  page reclaims
                   0  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                  59  voluntary context switches
                2209  involuntary context switches
         43564023536  instructions retired
         18132780797  cycles elapsed
              487424  peak memory footprint
```
- For a comparable [python implementation](./python/sequence_cleaner.py):
```sh
✗ time -hl python3 ./python/sequence_cleaner.py /Users/koohoko/Downloads/test.fasta 4500 ./data/BA1_BA2_pos.txt > /Users/koohoko/Downloads/benchmark_python.fasta 2> ./data/benchmark_python.log
	11.85s real		10.65s user		1.30s sys
            32690176  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                8678  page reclaims
                   6  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                 478  voluntary context switches
               47908  involuntary context switches
         96607991082  instructions retired
         40105094049  cycles elapsed
            23801856  peak memory footprint
```


#### TODO / PLANS
* [x] Test pipe streams. Stdin and Stdout work as expected.
* [x] Test zip files. gz and xz inputs are also supported.
* [x] Benchmark against python implementation.
* [ ] Improve performance.
* [ ] Work in multithread mode?


---

