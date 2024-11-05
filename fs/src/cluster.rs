pub trait Cluster {
    fn num_of_sectors(&self) -> usize;
    fn to_sector_index(&self) -> usize;
}
