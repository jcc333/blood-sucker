pub trait Serde {
    fn ser(&self) -> Vec<u8>;
    fn de(&[u8]) -> Result<(Self, u32), &str>;
}
