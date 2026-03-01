#![cfg_attr(not(feature = "std"), no_std)]
//! # KOMSI Library
//!
//! This crate provides a type save library for the KOMSI protocol, often used for vehicle telemetry
//! in simulators like "The Bus" or "OMSI 2".
//!
//! It includes structures for tracking vehicle state and functions for building and decoding KOMSI commands.

/// KOMSI protocol command types and builders.
pub mod komsi;
/// Vehicle state tracking and comparison.
pub mod vehicle;

pub use komsi::KomsiDateTime;
pub use komsi::KomsiCommand;
pub use komsi::KomsiError;
pub use vehicle::VehicleState;
