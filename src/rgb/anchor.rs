// LNP/BP Rust Library
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::io;

use bitcoin::Transaction;

use crate::bp::OuterWitness;
use crate::lnpbp4::{Lnpbp4Hash, MultimsgCommitment};
use crate::strict_encoding::{self, StrictDecode, StrictEncode};

#[derive(Clone, Debug)]
pub struct Anchor {
    pub commitments: Vec<Lnpbp4Hash>,
    pub entropy: Option<u64>,
    pub proof: OuterWitness,
}

impl Anchor {
    pub fn new(_lnpbp4: MultimsgCommitment, _tx: &Transaction) -> Self {
        unimplemented!()
    }
}

impl StrictEncode for Anchor {
    fn strict_encode<E: io::Write>(&self, _: E) -> Result<usize, strict_encoding::Error> {
        unimplemented!()
    }
}

impl StrictDecode for Anchor {
    fn strict_decode<D: io::Read>(_: D) -> Result<Self, strict_encoding::Error> {
        unimplemented!()
    }
}
