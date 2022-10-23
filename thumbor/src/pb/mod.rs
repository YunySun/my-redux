/*
 * @Description:
 * @Author: 李昶
 * @Date: 2022-10-23 11:46:38
 * @LastEditors: 李昶
 * @LastEditTime: 2022-10-23 17:06:24
 * @Profile: 一个比较废柴的前端开发
 */
use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use photon_rs::transform::SamplingFilter;
use prost::Message;
use std::convert::TryFrom;

mod abi; // 声明abi.rs
pub use abi::*;
impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

// 让ImageSpec 可以生成一个字符串
impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}

// 让ImageSpec可以通过一个字符串创建。比如s.parse().unwrap()
impl TryFrom<&str> for ImageSpec {
    type Error = any::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = decode_config(value, URL_SAFE_NO_PAD)?;
        Ok(ImageSpec::decode(&data[..])?)
    }
}

// 辅助函数，photo_rs相应的方法需要字符串
impl filter::Filter {
    pub fn to_str(&self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Islands => Some("islands"),
            filter::Filter::Marine => Some("marine"),
        }
    }
}

// 在我们定义的SampleFilter和photo_rs的SamplingFilter间转换
impl From<resize::SampleFilter> for SampleFilter {
    fn from(v: resize::SampleFilter) -> Self {
        match v {
            resize::SampleFilter::Undefined => SampleFilter::Nearest,
            resize::SampleFilter::Nearest => SampleFilter::Nearest,
            resize::SampleFilter::Triangle => SampleFilter::Triangle,
            resize::SampleFilter::CatmullRom => SampleFilter::CatmullRom,
            resize::SampleFilter::Gaussian => SampleFilter::Gaussian,
            resize::SampleFilter::Lancz0s3 => SampleFilter::Lancz0s3,
        }
    }
}

impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Date::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::SeamCrave as i32,
                filter: resize::SampleFilter::Undefined as i32,
            })),
        }
    }

    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            })),
        }
    }

    pub fn new_filter(filter: filter::Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter {
                filter: filter as i32,
            })),
        }
    }

    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark { x, y })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;
    use std::convert::TryInto;

    #[test]
    fn encoded_spec_could_be_decoded() {
        let spec1 = Spec::new_resize(600, 600, resize::SampleFilter::CatmullRom);
        let spec2 = Spec::new_filter(filter::Filter::Marine);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);
        let s: String = image_spec::borrow().into();
        assert_eq!(image_spec, s.as_str().try_into().unwrap());
    }
}
