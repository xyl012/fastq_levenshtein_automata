# Fastq Sequence Filtering with Levenshtein Automata (fuzzy string match)
Reads a fastq and finds a list of supplied sequences within a Levenshtein distance via finite state automata. Download a release for your OS and run from the command line without outside dependencies. This repo:
<ul>
<li>Uses seq io to stream a fastq efficiently</li>
<li>Creates a set within levenshtein distance (-d or --distance options) of the list of sequences from a fastq</li>
<li>Checks for the sequences and returns any fastq lines within distance in our fastq. </li>
</ul>

## Example

![fqlev](fqlev.gif)

We have a list of sequences that we want to find in our fastq 'in.fastq.gz' within our file named 'bc_4count.txt'. The contents of sequences.txt are three sequences separated by a return/newline:

AAAAAAAAAAAAAAAAAAAAAAAAAAAAAA <br>
AAAAAAAAAAAAAAAAAAAAAAAAAAAGGG <br>
AAAAAAAAAAAAAAAAAAAAAAAAAAACCC <br>

Our output will be out.fastq

Our command : `./fastq_levenshtein_automata --fastq=in.fastq.gz --sequence_list=bc_4count.txt --output=out.fastq`


## Previous Work

This tool is simply an implementation of [Automata by burnt sushi](https://blog.burntsushi.net/transducers/)
 to fastqs. There are a lot of implementations to find hamming or levenshtein distance between single strings, but I wanted to find levenshtein distance of a list of sequences in a fastq and make a new fastq in a streaming fashion.


## Notes

The sequences in the list compare against the whole sequence in the fastq, e.g. if the sequence of the fastq is 5 A's and your list has 3 A's, it won't find it if your levenshtein distance is 1.
