// LNP/BP Core Library implementing LNPBP specifications & standards
// Written in 2019 by
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

// Coding conventions
#![recursion_limit = "256"]
#![deny(dead_code, missing_docs, warnings)]
// TODO #184: when we will be ready for the release #![deny(missing_docs)]

#[macro_use]
extern crate amplify;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_with;
#[cfg(feature = "serde")]
extern crate serde_crate as serde;

pub extern crate bech32;
pub extern crate chain;
#[cfg(feature = "elgamal")]
pub extern crate elgamal;
pub mod tagged_hash;

pub extern crate client_side_validation;
pub use client_side_validation::commit_encode_list;
pub use client_side_validation::commit_verify;
#[macro_use]
pub extern crate strict_encoding;
pub use strict_encoding::{
    impl_enum_strict_encoding, strict_decode_self, strict_encode_list,
    test_encode, test_enum_u8_exhaustive, test_garbage_exhaustive,
};
#[macro_use]
pub extern crate strict_encoding_derive;
pub use strict_encoding_derive::{StrictDecode, StrictEncode};

pub use chain::{Chain, P2pNetworkId};
pub use seals::TxoutSeal;
pub use seals::{lnpbp1, lnpbp2, lnpbp3, lnpbp4};
pub use short_id::ShortId;
pub use tagged_hash::TaggedHash;

#[cfg(test)]
pub mod test {
    use bitcoin::secp256k1;
    use wallet::SECP256K1;

    pub fn gen_secp_pubkeys(n: usize) -> Vec<secp256k1::PublicKey> {
        let mut ret = Vec::with_capacity(n);
        let mut sk = [0; 32];

        for i in 1..n + 1 {
            sk[0] = i as u8;
            sk[1] = (i >> 8) as u8;
            sk[2] = (i >> 16) as u8;

            ret.push(secp256k1::PublicKey::from_secret_key(
                &SECP256K1,
                &secp256k1::SecretKey::from_slice(&sk[..]).unwrap(),
            ));
        }
        ret
    }

    pub fn gen_bitcoin_pubkeys(
        n: usize,
        compressed: bool,
    ) -> Vec<bitcoin::PublicKey> {
        gen_secp_pubkeys(n)
            .into_iter()
            .map(|key| bitcoin::PublicKey { key, compressed })
            .collect()
    }
}
