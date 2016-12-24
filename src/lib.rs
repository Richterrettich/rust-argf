use std::io;
use std::io::Read;
use std::iter::Iterator;

pub fn from_arguments() -> Box<io::Read> {

    return from_slice(&std::env::args().collect::<Vec<String>>());
}

pub fn from_slice<I>(input: &[I]) -> Box<io::Read>
    where I: AsRef<str>
{
    let mut chain: Box<Read> = Box::new(std::io::empty().chain(std::io::empty()));
    for reader in input.iter().map(|p| to_reader(p.as_ref())) {
        chain = Box::new(chain.chain(reader));
    }
    chain
}

fn to_reader<'a>(path: &'a str) -> Box<io::Read> {
    if path == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(std::fs::File::open(path).unwrap())
    }
}
