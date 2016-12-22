use std::io;
use std::io::Read;
use std::iter::Iterator;

pub fn from_arguments() -> Box<io::Read> {
    return from_iterator(std::env::args().skip(1));
}

pub fn from_iterator<I>(iter: I) -> Box<io::Read>
    where I: Iterator<Item = String>
{
    let mut chain: Box<Read> = Box::new(std::io::empty().chain(std::io::empty()));
    for reader in iter.map(|p| to_reader(p)) {
        chain = Box::new(chain.chain(reader));
    }
    chain
}

fn to_reader(path: String) -> Box<io::Read> {
    if path == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(std::fs::File::open(path).unwrap())
    }
}
