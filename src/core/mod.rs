pub mod config;
pub mod net;

pub mod lock;
pub(crate) use lock::lock_fief;

use anyhow::Result;
use image::{ImageFormat, ImageReader};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tap::prelude::*;

pub const WPLACE_CHUNK_WIDTH: usize = 1000;
pub const WPLACE_CHUNK_HEIGHT: usize = 1000;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<[usize; 2]> for Position {
    fn from(value: [usize; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct ImagePng(Vec<u8>);

impl ImagePng {
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    pub fn new(raw_data: Vec<u8>) -> Self {
        raw_data.pipe(Self)
    }

    fn to_reader(self) -> ImageReader<Cursor<Vec<u8>>> {
        self.0
            .pipe(Cursor::new)
            .pipe(ImageReader::new)
            .tap_mut(|r| r.set_format(ImageFormat::Png))
    }

    pub fn try_to_rgba(self) -> Result<image::RgbaImage> {
        let result = self.to_reader().decode()?.to_rgba8();
        Ok(result)
    }

    pub fn try_to_gray(self) -> Result<image::GrayImage> {
        let result = self.to_reader().decode()?.to_luma8();
        Ok(result)
    }

    pub fn try_from_rgba(value: image::RgbaImage) -> Result<Self> {
        let mut cursor = Cursor::new(Vec::<u8>::new());
        value.write_to(&mut cursor, image::ImageFormat::Png)?;
        Ok(Self(cursor.into_inner()))
    }

    pub fn try_from_gray(value: image::GrayImage) -> Result<Self> {
        let mut cursor = Cursor::new(Vec::<u8>::new());
        value.write_to(&mut cursor, image::ImageFormat::Png)?;
        Ok(Self(cursor.into_inner()))
    }
}

impl std::fmt::Debug for ImagePng {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ImagePng(bytes...)")
    }
}

impl TryFrom<ImagePng> for image::RgbaImage {
    type Error = anyhow::Error;

    fn try_from(value: ImagePng) -> std::result::Result<Self, Self::Error> {
        value.try_to_rgba()
    }
}

impl TryFrom<ImagePng> for image::GrayImage {
    type Error = anyhow::Error;

    fn try_from(value: ImagePng) -> std::result::Result<Self, Self::Error> {
        value.try_to_gray()
    }
}

impl TryFrom<image::RgbaImage> for ImagePng {
    type Error = anyhow::Error;

    fn try_from(value: image::RgbaImage) -> std::result::Result<Self, Self::Error> {
        Self::try_from_rgba(value)
    }
}

impl TryFrom<image::GrayImage> for ImagePng {
    type Error = anyhow::Error;

    fn try_from(value: image::GrayImage) -> std::result::Result<Self, Self::Error> {
        Self::try_from_gray(value)
    }
}
