# fasta_windows

To kick of the new blog series, I thought I'd talk about my first serious Rust project, <a href="https://github.com/tolkit/fasta_windows">fasta_windows</a>. It's a pretty simple tool and most of its functionality is covered already by a wide range of bioinformatic software tools. However, as the great Heng Li once said - <a href="https://twitter.com/lh3lh3/status/1451600007115780098">"To tool developers, reinventing wheels is an important skill."</a>. And at this point, I wanted to get Rust under my belt. So let's get to the why and how.

## Why?

I am currently working as part of the <a href="https://www.darwintreeoflife.org/">Darwin Tree of Life</a> project, which aims to sequence the genome of every eukaryotic organism in UK and Ireland to chromosomal quality. There are around 70,000 species, and the first <a href="https://www.darwintreeoflife.org/genomes/genome-notes/">100 genome notes</a> have just been published (with more in public databases). For these genomes, it would be nice to have a first pass analysis and visualisation for some basic statistics. As they are chromosomal, this makes visualisation even nicer! Eventually, I hope these statistic files, which will be TSV's, will be deposited somewhere anyone can get their hands on them. 

## How?

The Darwin Tree of Life genomes are emitted as fasta files. The fasta file format is probably one of the most ubiquitous file formats in bioinformatics. It's easy to generate and parse, and has only really two rules:

- A header line which starts with a `>`
- A sequence line(s) immediately below

```txt
>header (you can put anything here you want. As long as it's valid UTF-8)
ACTATTCACTTT
>another header
CACACACATTTA
```

We can chop each of the sequences in a fasta file into chunks (non-overlapping in this case), and compute statistics on these chunks. Sounds simple enough! And it is really... let's dive into the Rust implementation details.


## Rust implementation details

<a href="https://github.com/rust-bio/rust-bio">Rust Bio</a> is an excellent Rust crate providing I/O functionality for many common file types found in computational biology, amongst other things. It's not *the fastest* solution out there for parsing fastas, but it's well documented and has nice API's.

(Disclaimer: I'm poaching this from the <a href="https://github.com/tolkit/fasta_windows">fasta_windows</a> GitHub page, and haven't `cargo check`-ed the following code.)

```rust
// we can use it like this
use bio::io::fasta;

// and make a reader
let input_fasta = "/path/to/some/fasta/on/disk.fasta";
let reader = fasta::Reader::from_file(input_fasta).expect("Path invalid.");
```

The <a href="https://github.com/rayon-rs/rayon">rayon</a> Rust crate can easily turn a normal Rust iterator into a parallel one. We can leverage this when we want to iterate over all of our fasta records.

```rust
use std::sync::mpsc::channel;
// got to make a sender and receiver channel
let (sender, receiver) = channel();

// use the reader from above
reader
    // records() is the iterator over fasta records
    .records()
    // magical parallel iterator
    .par_bridge()
    // this is where we do the hard work
    .for_each_with(sender, |s, record| {
        // we can access the fasta record here
        let fasta_record = record.expect("Error during fasta record parsing.");

        // optionally we can use the sender to send data through a 
        // thread safe channel
        // let's send a tuple of our fasta headers and seq lens... 
        s.send((fasta_record.id(), fasta_record.seq().len()))
    });

// outside the parallel iterator now, phew.
// let's collect our parallel booty into a vector
let collection: Vec<(&str, usize)> = receiver.iter().collect();

// why not print it out
for (id, length) in collection {
    println!("Record: {}\nSequence length: {}", id, length);
}

```

This is essentially the parallel iterator implementation in `fasta_windows`. Now to briefly mention how the statistics are generated in windows. Rust has a useful <a href="https://doc.rust-lang.org/std/primitive.slice.html#method.chunks">chunks</a> function, which operates on slices. As Rust Bio gives us back UTF-8 checked byte slices, this is perfect for what we need. `chunks` gives non-overlapping windows.

```rust
// fasta_record comes from above, it's of type 
// bio::io::fasta::Record
// the seq method returns a `TextSlice<'_>
// which is just an alias for & [u8]

// 1kb windows
let window_size = 1000;

// make the chunks iterator
// (actually going through this code has made me think
// that a more elegant solution can be found here
// with the `chunks_exact` method...)
let windows = fasta_record.seq().chunks(window_size);

// now we can do the interesting stuff

// this is not how I did it, but for argument sake
// we could make a GC struct
struct GC {
    percent: f32,
}

impl GC {
    fn calc(&self, dna: &[u8]) -> Self {
        // actually do the calculation
        Self { percent: ... }
    }
}

for win in windows {
    // calculate GC content etc here.
    let gc = GC::calc(win);

    // if this block is contained in the parallel
    // for_each_with above, we could send this 
    // gc result through this.
    s.send(gc)
}
```

There is some window logic to handle, like keeping track of the windows (what if they are too long or too short?), and some other statistics I calculate. The above code however is the main logic of the program! Delve into the source code at the repository to explore further (<a href="https://github.com/tolkit/fasta_windows">**here**</a>). Or give it a go, and tell me what you think!