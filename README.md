### fasta_filter

This little tool accept a fasta file (or from stdin), and out put a filtered fasta file. 

**Usage:**
`fasta_filter [flags] `

- Flags:
```
-f --file [FILE]
		Path of fasta file or use '-' as stdin

-b --base [string]	
		Bases to be accounted for. Examples: "N,-,?" [Default: 'N'].

-n --num_base [Number] 
		Number of specified bases, any sequences with bases 
		count over this threshold will not be print out.

-s --specified_pos_file [FILE]	
		Path to a txt file specifying genomic positions of interest,
		each line should contain one integer specifying nucleotide position.

--specified_num_base [Number]	
		The num_base threshold for the specified positions.
```

**TODO / PLANS**
* [ ] Test zip files.
* [ ] Test pipe streams.
* [ ] Work in multithread mode.
* [ ] Benchmark against python implementation.

---

