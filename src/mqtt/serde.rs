use std::io::Read;
use std::io::Result;
use std::io::Write;

pub trait Serde: Sized {
    // Returns either an error, or the bytes written to `sink`
    fn ser(&self, sink: &mut Write) -> Result<usize>;

    // Returns either an error, or a constructed object and the bytes consumed from `source`
    fn de(source: &mut Read) -> Result<(Self, usize)>;
}
