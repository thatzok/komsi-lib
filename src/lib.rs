//! # KOMSI Library
//!
//! This crate provides a library for the KOMSI protocol, often used for vehicle telemetry
//! in simulators like "The Bus" or "OMSI 2"
//!
//! It includes structures for tracking vehicle state and functions for building KOMSI commands.

/// KOMSI protocol command types and builders.
pub mod komsi;
/// Vehicle state tracking and comparison.
pub mod vehicle;
