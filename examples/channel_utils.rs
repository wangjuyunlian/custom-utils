use anyhow::{bail, Result};
use custom_utils::{rx, rx_async, tx, tx_async};
use log::error;
use std::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<()> {
    custom_utils::logger::logger_stdout_debug();
    assert!(test_sync_right().is_ok());
    assert!(test_tx().is_err());
    assert!(test_tx_msg().is_err());
    assert!(test_rx().is_err());
    assert!(test_rx_msg().is_err());

    assert!(test_async_right().await.is_ok());
    assert!(test_tx_async().await.is_err());
    assert!(test_tx_async_msg().await.is_err());
    assert!(test_rx_async().await.is_err());
    assert!(test_rx_async_msg().await.is_err());
    Ok(())
}
fn test_sync_right() -> Result<()> {
    let (tx, rx) = channel::<()>();
    tx!(tx, ());
    Ok(rx!(rx))
}
fn test_tx() -> Result<()> {
    let (tx, rx) = channel::<()>();
    drop(rx);
    tx!(tx, ());
    Ok(())
}
fn test_tx_msg() -> Result<()> {
    let (tx, rx) = channel::<()>();
    drop(rx);
    tx!(tx, (), "something error!");
    Ok(())
}
fn test_rx() -> Result<()> {
    let (tx, rx) = channel::<()>();
    drop(tx);
    Ok(rx!(rx))
}
fn test_rx_msg() -> Result<()> {
    let (tx, rx) = channel::<()>();
    drop(tx);
    Ok(rx!(rx, "receive fail"))
}

async fn test_async_right() -> Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(3);
    tx_async!(tx, ());
    Ok(rx_async!(rx))
}
async fn test_tx_async() -> Result<()> {
    let (tx, rx) = tokio::sync::mpsc::channel::<()>(3);
    drop(rx);
    tx_async!(tx, ());
    Ok(())
}
async fn test_tx_async_msg() -> Result<()> {
    let (tx, rx) = tokio::sync::mpsc::channel::<()>(3);
    drop(rx);
    tx_async!(tx, (), "async something error!");
    Ok(())
}
async fn test_rx_async() -> Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(3);
    drop(tx);
    Ok(rx_async!(rx))
}
async fn test_rx_async_msg() -> Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(3);
    drop(tx);
    Ok(rx_async!(rx, "async receive fail"))
}
