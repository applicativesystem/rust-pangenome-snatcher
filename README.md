# rust-pangenome-snatcher
 - rust pangenome snatcher takes a pangenome alignment and the reference genome and the query genome file and write the pangenome aligned files. 

 ```
 cargo build 
 
 ```

```
λ gauravsablok rust-pangenome-snatcher → λ git main* → ./rust-pangenome-snatcher -h
Usage: rust-pangenome-snatcher <PAFALIGNMENT> <PAF_QUERY_FASTA> <PAF_REFERENCE_FASTA>

Arguments:
  <PAFALIGNMENT>         please provide the path to the alignment file
  <PAF_QUERY_FASTA>      please provide the path to the fasta file
  <PAF_REFERENCE_FASTA>  please provide the path to the reference file

Options:
  -h, --help     Print help
  -V, --version  Print version
 
```
 - how to run the binary 

```
./rust-pangenome-snatcher ./sample-files/pangenome.paf ./sample-files/pangenome_query.fasta ./sample-files/pangenome_ref.fasta

```

 Gaurav Sablok
