/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::{
    errors::{elosysError, elosysErrorKind},
    serializing::{bytes_to_hex, hex_to_bytes},
};
use group::GroupEncoding;
use elosys_zkp::constants::PUBLIC_KEY_GENERATOR;
use jubjub::SubgroupPoint;

use std::io;

use super::{IncomingViewKey, SaplingKey};
pub const PUBLIC_ADDRESS_SIZE: usize = 32;

/// The address to which funds can be sent, stored as a public
/// transmission key. Using the incoming_viewing_key allows
/// the creation of a unique public addresses without revealing the viewing key.
#[derive(Clone, Copy)]
pub struct PublicAddress(pub(crate) SubgroupPoint);

impl PublicAddress {
    /// Initialize a public address from its 32 byte representation.
    pub fn new(
        public_address_bytes: &[u8; PUBLIC_ADDRESS_SIZE],
    ) -> Result<PublicAddress, elosysError> {
        assert!(public_address_bytes.len() == 32);
        let public_address_non_prime = SubgroupPoint::from_bytes(public_address_bytes);

        if public_address_non_prime.is_some().into() {
            Ok(PublicAddress(public_address_non_prime.unwrap()))
        } else {
            Err(elosysError::new(elosysErrorKind::InvalidPaymentAddress))
        }
    }

    /// Load a public address from a Read implementation (e.g: socket, file)
    pub fn read<R: io::Read>(reader: &mut R) -> Result<Self, elosysError> {
        let mut address_bytes = [0; PUBLIC_ADDRESS_SIZE];
        reader.read_exact(&mut address_bytes)?;
        Self::new(&address_bytes)
    }

    /// Initialize a public address from a sapling key. Typically constructed from
    /// SaplingKey::public_address()
    pub fn from_key(sapling_key: &SaplingKey) -> PublicAddress {
        Self::from_view_key(sapling_key.incoming_view_key())
    }

    pub fn from_view_key(view_key: &IncomingViewKey) -> PublicAddress {
        PublicAddress(*PUBLIC_KEY_GENERATOR * view_key.view_key)
    }

    /// Convert a String of hex values to a PublicAddress. The String must
    /// be 64 hexadecimal characters representing the 32 bytes of an address
    /// or it fails.
    pub fn from_hex(value: &str) -> Result<Self, elosysError> {
        match hex_to_bytes(value) {
            Err(_) => Err(elosysError::new(elosysErrorKind::InvalidPublicAddress)),
            Ok(bytes) => Self::new(&bytes),
        }
    }

    /// Retrieve the public address in byte form.
    pub fn public_address(&self) -> [u8; PUBLIC_ADDRESS_SIZE] {
        self.0.to_bytes()
    }

    /// Retrieve the public address in hex form.
    pub fn hex_public_address(&self) -> String {
        bytes_to_hex(&self.public_address())
    }

    /// Store the bytes of this public address in the given writer.
    pub fn write<W: io::Write>(&self, mut writer: W) -> Result<(), elosysError> {
        writer.write_all(&self.public_address())?;

        Ok(())
    }
}

impl std::fmt::Debug for PublicAddress {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "PublicAddress {}", self.hex_public_address())
    }
}

impl std::cmp::PartialEq for PublicAddress {
    fn eq(&self, other: &Self) -> bool {
        self.hex_public_address() == other.hex_public_address()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        keys::{PublicAddress, PUBLIC_ADDRESS_SIZE},
        SaplingKey,
    };

    #[test]
    fn public_address_validation() {
        let bad_address = "8a4685307f159e95418a0dd3d38a3245f488c1baf64bc914f53486efd370c562";
        let good_address = "8a4685307f159e95418a0dd3d38a3245f488c1baf64bc914f53486efd370c563";

        let bad_result = PublicAddress::from_hex(bad_address);
        assert!(bad_result.is_err());

        PublicAddress::from_hex(good_address).expect("returns a valid public address");
    }

    #[test]
    fn public_address_generation() {
        let sapling_key = SaplingKey::generate_key();
        let public_address = sapling_key.public_address();
        assert_eq!(public_address.public_address().len(), PUBLIC_ADDRESS_SIZE);
    }
}
