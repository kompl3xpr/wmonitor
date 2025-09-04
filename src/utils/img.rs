use image::{ImageReader, ImageFormat};
use std::io::Cursor;
use tap::prelude::*;
use anyhow::Result;

pub struct ImagePng(Vec<u8>);

impl ImagePng {
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    pub fn new(raw_data: Vec<u8>) -> Self {
        raw_data
            .pipe(Self)
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