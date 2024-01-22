//! module to iterator through a given epoch
//!
//! The iterator stops once the epoch is completely traversed.
//!

use cardano::block::{EpochId, RawBlock};
use storage_units::packfile;

use super::super::StorageConfig;
use epoch::{epoch_open_pack_reader, epoch_read_pack};
use pack::packreader_init;

use std::fs;

use super::Result;

/// Iterator over every blocks of a given epoch
pub struct Iter(packfile::Reader<fs::File>);
impl Iter {
    pub fn new(storage: &StorageConfig, epoch: EpochId) -> Result<Self> {
        let packref = epoch_read_pack(storage, epoch)?;
        let reader = packreader_init(&storage, &packref);
        Ok(Iter(reader))
    }

    // TODO:
    // * pub fn from_hash(mut self, hash: HeaderHash) -> Self {}
    // * pub fn from_slot(mut self, slot: u32) -> Self;
}
impl Iterator for Iter {
    type Item = Result<RawBlock>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next_block() {
            Ok(None) => None,
            Ok(Some(v)) => Some(Ok(RawBlock(v))),
            Err(e) => Some(Err(e.into())),
        }
    }
}

/// Create an iterator over every epoch of the storage
///
/// The iterator will returns `Iter` so it is possible to iterate
/// over the block from it.
pub struct Epochs<'a> {
    storage_config: &'a StorageConfig,

    epoch_id: EpochId,
}
impl<'a> Epochs<'a> {
    pub fn new(storage: &'a StorageConfig) -> Self {
        Epochs {
            storage_config: storage,
            epoch_id: 0,
        }
    }

    pub fn from_epoch(mut self, epoch_id: EpochId) -> Self {
        self.epoch_id = epoch_id;
        self
    }
}
impl<'a> Iterator for Epochs<'a> {
    type Item = Result<Iter>;

    fn next(&mut self) -> Option<Self::Item> {
        let r = epoch_open_pack_reader(&self.storage_config, self.epoch_id);
        match r {
            Err(e) => Some(Err(e)),
            Ok(None) => None,
            Ok(Some(r)) => {
                let iter = Iter(r);
                self.epoch_id += 1;
                Some(Ok(iter))
            }
        }
    }
}
