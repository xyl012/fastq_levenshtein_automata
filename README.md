# fastq_levenshtein_automata
Reads a fastq and finds a list of supplied sequences within a Levenshtein distance via finite state automata. Download a release for your OS and run from the command line without outside dependencies. This repo:
<ul>
<li>Uses seq io to efficiently read a fastq </li>
<li>Creates a set within levenshtein distance (-d or --distance options) of the list of sequences from a fastq</li>
<li>Checks for the sequences and returns any fastq lines within distance in our fastq. </li>
</ul>

## Example

We have a list of sequences that we want to find in our fastq 'in.fastq.gz' within our file named 'sequences.txt'. The contents of sequences.txt are three sequences separated by a return/newline:

AAAAAAAAAAAAAAAAAAAAAAAAAAAAAA

AAAAAAAAAAAAAAAAAAAAAAAAAAAGGG

AAAAAAAAAAAAAAAAAAAAAAAAAAACCC

Our output will be out.fastq

Our command : `fastq_levenshtein_automata --fastq=in.fastq.gz --sequence_list=sequences.txt --output=out.fastq`


## Previous Work

This tool is simply an implementation of [Automata by burnt sushi](https://blog.burntsushi.net/transducers/)
 to fastqs. There are a lot of ways to find hamming or levenshtein distance between single strings, but I wanted to find levenshtein distance of a list of sequences in a fastq and make a new fastq.
