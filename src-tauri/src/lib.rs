#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::collapsible_if)]

pub mod backtest;
pub mod bot;
pub mod config;
pub mod error;
pub mod exchange;
pub mod optimize;
pub mod persistence;
pub mod risk;
pub mod risk_commands;
pub mod strategy;
pub mod tests;
pub mod trade;
pub mod types;

pub use error::{AppError, Result};
