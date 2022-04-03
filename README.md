### fasta_filter

This little tool accept a fasta file (or from stdin), and out put a filtered fasta file. 

**Usage:**
`fasta_filter [flags] INPUT`
- INPUT can be a fasta file or stream from stdin.

- Flags:
	```
	-base [string]	
			Bases to be accounted for. Examples: "N,-,?" [Default: 'N'].

	-freq_base [float] 
			Frequency of specified bases, any sequences with bases 
			frequency over this threshold will not be print out.

	-num_base [int]	
			Number of specified bases, any sequences with bases count
			over this threshold will not be print out.

	-specified_pos [txt file]	
			A txt file specifying genomic positions of interest,
			each line should contain one integer specifying nucleotide position.

	-specified_freq_base [float]	
			The freq_base threshold for the specified positions.

	-specified_num_base [int]	
			The num_base threshold for the specified positions.
	```

**TODO / PLANS**
* [ ] Work in multithread mode.
* [ ] Benchmark against python implementation.

---

