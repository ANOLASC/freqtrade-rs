#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod error;
pub mod types;
pub mod config;
pub mod persistence;
pub mod exchange;
pub mod strategy;
pub mod backtest;
pub mod bot;
pub mod risk;
pub mod risk_commands;
pub mod optimize;

pub use error::{AppError, Result};
