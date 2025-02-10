// Copyright lowRISC contributors (OpenTitan project).
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// This file was autogenerated by `//sw/host/hsmtool/scripts/pkcs11_consts.py`.
// Do not edit.'

use cryptoki_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::convert::TryFrom;

use crate::util::attribute::{AttrData, AttributeError};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    IntoPrimitive,
    FromPrimitive,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(u64)]
pub enum CertificateType {
    #[serde(rename = "CKC_X_509")]
    X509 = CKC_X_509,
    #[serde(rename = "CKC_X_509_ATTR_CERT")]
    X509AttrCert = CKC_X_509_ATTR_CERT,
    #[serde(rename = "CKC_WTLS")]
    Wtls = CKC_WTLS,
    #[serde(rename = "CKC_VENDOR_DEFINED")]
    VendorDefined = CKC_VENDOR_DEFINED,
    #[num_enum(catch_all)]
    UnknownCertificateType(u64) = u64::MAX,
}

impl From<cryptoki::object::CertificateType> for CertificateType {
    fn from(val: cryptoki::object::CertificateType) -> Self {
        let val = CK_CERTIFICATE_TYPE::from(val);
        Self::from(val)
    }
}

impl TryFrom<CertificateType> for cryptoki::object::CertificateType {
    type Error = cryptoki::error::Error;
    fn try_from(val: CertificateType) -> Result<Self, Self::Error> {
        let val = CK_CERTIFICATE_TYPE::from(val);
        cryptoki::object::CertificateType::try_from(val)
    }
}

impl TryFrom<&AttrData> for CertificateType {
    type Error = AttributeError;
    fn try_from(val: &AttrData) -> Result<Self, Self::Error> {
        match val {
            AttrData::CertificateType(x) => Ok(*x),
            _ => Err(AttributeError::EncodingError),
        }
    }
}

impl From<CertificateType> for AttrData {
    fn from(val: CertificateType) -> Self {
        AttrData::CertificateType(val)
    }
}
