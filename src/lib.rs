// #![deny(warnings, missing_docs)]
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]

//! Parse [Dat protocol SLEEP
//! files](https://github.com/datproject/docs/blob/master/papers/sleep.md).
//!
//! ## Format
//!
//! ```txt,ignore
//! <32 byte header>
//!   <4 byte magic string: 0x05025702>
//!   <1 byte version number: 0>
//!   <2 byte entry size: 40>
//!   <1 byte algorithm name length prefix: 7>
//!   <7 byte algorithm name: BLAKE2b>
//!   <17 zeroes>
//! <40 byte entries>
//!   <32 byte BLAKE2b hash>
//!   <8 byte Uint64BE children leaf byte length>
//! ```

use std::error::Error;
use std::io;

macro_rules! bail {
  ($msg: expr) => {
    return Err(Box::new(io::Error::new(
      io::ErrorKind::Other,
      $msg,
    )));
  };
}

macro_rules! ensure {
  ($cond: expr, $msg: expr) => {
    if !($cond) {
      bail!($msg);
    }
  };
}

/// Algorithm used for hashing the data.
pub enum HashAlgorithm {
  /// [BLAKE2b](https://blake2.net/) hashing algorithm.
  BLAKE2b,
  /// [Ed25519](https://ed25519.cr.yp.to/) hashing algorithm.
  Ed25519,
}

/// Type of file.
pub enum FileType {
  BitField,
  Signatures,
  Tree,
}

/// SLEEP Protocol version.
pub enum Version {
  V0,
}

/// Struct representation of 32 byte SLEEP headers.
pub struct Header {
  pub file_type: FileType,
  pub version: Version,
  pub entry_size: u16,
  pub hash_algorithm: HashAlgorithm,
}

impl Header {
  pub fn new(
    tree_type: FileType,
    entry_size: u16,
    hash_algorithm: HashAlgorithm,
  ) {
  }

  /// Parse a 32 bit buffer into a valid Header type.
  pub fn from_vec(buffer: &Vec<u8>) -> Result<Header, Box<Error>> {
    ensure!(
      buffer.len() == 32,
      "buffer should be at least 32 bytes"
    );
    ensure!(
      buffer[0] == 5,
      "The first byte of a SLEEP header should be '5' (hex '0x05')"
    );
    ensure!(
      buffer[1] == 2,
      "The second byte of a SLEEP header should be '2' (hex '0x02')"
    );
    ensure!(
      buffer[2] == 87,
      "The third byte of a SLEEP header should be '87' (hex '0x57')"
    );

    let file_type = match buffer[3] {
      0 => FileType::BitField,
      1 => FileType::Signatures,
      2 => FileType::Tree,
      num => bail!(format!(
        "The byte '{}' does not belong to any known SLEEP file type",
        num
      )),
    };

    Ok(Header {
      version: Version::V0,
      entry_size: 40,
      file_type: file_type,
      hash_algorithm: HashAlgorithm::BLAKE2b,
    })
  }

  /// Convert a `Header` into a `Vec<u8>`. Use this to persist a header back to
  /// disk.
  pub fn to_vec(&self) {}
}

#[test]
fn test() {
  use std::fs::File;
  use std::io::{BufRead, BufReader};

  let file = File::open("README.md").unwrap();
  let mut reader = BufReader::with_capacity(40, file);
  let buffer = reader.fill_buf().unwrap();
  println!("{:?}", buffer.len());
}