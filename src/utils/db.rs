use sqlx::{Decode, Encode, Type};
use std::io::Cursor;
use std::fmt::{Debug, Formatter};

pub type CurrentDb = sqlx::Sqlite;
pub type CurrentRow = <CurrentDb as sqlx::Database>::Row;
pub type CurrentTypeInfo = <CurrentDb as sqlx::Database>::TypeInfo;

pub struct RgbaImg(pub image::RgbaImage);

impl Debug for RgbaImg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<RgbaImg>")
    }
}

impl From<image::RgbaImage> for RgbaImg {
    fn from(value: image::RgbaImage) -> Self {
        RgbaImg(value)
    }
}

impl Type<CurrentDb> for RgbaImg {
    fn type_info() -> CurrentTypeInfo {
        <Vec<u8> as Type<CurrentDb>>::type_info()
    }
}

impl<'q> Encode<'q, CurrentDb> for RgbaImg {
    fn encode_by_ref(
        &self,
        buf: &mut <CurrentDb as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let mut cursor = Cursor::new(Vec::<u8>::new());
        self.0.write_to(&mut cursor, image::ImageFormat::Png)?;
        Encode::<CurrentDb>::encode_by_ref(cursor.get_ref(), buf)
    }
}

impl<'r> Decode<'r, CurrentDb> for RgbaImg {
    fn decode(
        value: <CurrentDb as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let raw_data = <Vec<u8> as Decode<CurrentDb>>::decode(value)?;
        let mut reader = image::ImageReader::new(Cursor::new(raw_data));
        reader.set_format(image::ImageFormat::Png);
        let image = reader.decode()?.into_rgba8();
        Ok(RgbaImg(image))
    }
}

pub struct GrayImg(pub image::GrayImage);

impl Debug for GrayImg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<GrayImg>")
    }
}

impl From<image::GrayImage> for GrayImg {
    fn from(value: image::GrayImage) -> Self {
        GrayImg(value)
    }
}

impl Type<CurrentDb> for GrayImg {
    fn type_info() -> CurrentTypeInfo {
        <Vec<u8> as Type<CurrentDb>>::type_info()
    }
}

impl<'q> Encode<'q, CurrentDb> for GrayImg {
    fn encode_by_ref(
        &self,
        buf: &mut <CurrentDb as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let mut cursor = Cursor::new(Vec::<u8>::new());
        self.0.write_to(&mut cursor, image::ImageFormat::Png)?;
        Encode::<CurrentDb>::encode_by_ref(cursor.get_ref(), buf)
    }
}

impl<'r> Decode<'r, CurrentDb> for GrayImg {
    fn decode(
        value: <CurrentDb as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let raw_data = <Vec<u8> as Decode<CurrentDb>>::decode(value)?;
        let mut reader = image::ImageReader::new(Cursor::new(raw_data));
        reader.set_format(image::ImageFormat::Png);
        let image = reader.decode()?.into_luma8();
        Ok(GrayImg(image))
    }
}
