rust
use std::{
    collections::HashMap, 
    io,
    sync::Mutex, 
    path::PathBuf
};
use reqwest::Response;
use hyper;
use slog::Logger;

pub struct Module;
pub struct DownloaderError;

pub trait Downloader {
    fn get<'life0, 'life1, 'async_trait>(
        &'life0 self,
        url: &'life1 str,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Response, DownloaderError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;
}

pub struct Fetcher<T>
where
    T: Downloader,
{
    downloader: T,
    storage_folder: PathBuf,
    download_cache: Mutex<HashMap<[u8; 32], Vec<u8>>>,
}

impl<T> Fetcher<T>
where
    T: Downloader,
{
    pub fn new(downloader: T, storage_folder: PathBuf) -> Fetcher<T> {
        Self {
            downloader,
            storage_folder,
            download_cache: Mutex::new(HashMap::new()),
        }
    }

    pub async fn fetch_module(
        &self,
        logger: Logger,
        m: &Module,
    ) -> Result<PathBuf, DownloaderError> {
        Ok(PathBuf::new())
    }
}
