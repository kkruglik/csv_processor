use crate::{CellValue, CsvError, Dtype};

pub trait ChunkAgg<T> {
    fn sum(&self) -> Option<T> {
        None
    }
    fn min(&self) -> Option<T> {
        None
    }
    fn max(&self) -> Option<T> {
        None
    }
    fn mean(&self) -> Option<T> {
        None
    }

    // fn median(&self) -> Result<Option<T>, CsvError>;
    // fn mode(&self) -> Result<Option<T>, CsvError>;
    // fn variance(&self) -> Result<Option<T>, CsvError>;
}
