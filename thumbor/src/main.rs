/*
 * @Description:
 * @Author: 李昶
 * @Date: 2022-10-22 17:18:57
 * @LastEditors: 李昶
 * @LastEditTime: 2022-10-23 17:21:23
 * @Profile: 一个比较废柴的前端开发
 */
mod pb;
// 解析出来的图片处理的参数
// struct ImageSpec {
//     specs: Vec<Spec>,
// }

// // 每个参数是支持的某种方式
// enum Spec {
//     Resize(Resize),
//     Crop(Crop),
// }

// // 处理图片的resize
// struct Resize {
//     width: u32,
//     height: u32,
// }

// message ImageSpec { repeated Spec specs = 1; }

// message Spec {
//     oneof data {
//         Resize resize = 1;
//         Crop crop = 2;
//     }
// }

// fn print_test_url(url: &str) {
//     use std::borrow::Borrow;
//     let spec1 = Spec::new_resize(600, 800, resize::SampleFilter::CatmullRom);
//     let spec2 = Spec::new_watermark(20, 20);
//     let spec3 = Spec::new_filter(filter::Filter::Marine);
//     let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);
//     let s: String = image_spec.borrow().into();
//     let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
//     println!("test url: http://localhost:3000/image/{}/{}", s, test_image);
// }

fn main() {
    println!("Hello, world!");
}
