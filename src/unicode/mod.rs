//! Unicode-aware identifier support.
//!
//! This module is available with the `unicode` cargo feature and contains
//! Unicode comparison policies and security helpers. The Turkic-aware
//! policies additionally require the `unicode-turkic` cargo feature, which
//! pulls in ICU case-mapping data.

pub mod policy;
pub mod security;
