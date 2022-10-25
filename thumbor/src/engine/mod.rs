/*
 * @Description: c
 * @Author: 李昶
 * @Date: 2022-10-25 10:22:30
 * @LastEditors: 李昶
 * @LastEditTime: 2022-10-25 10:29:59
 * @Profile: 一个比较废柴的前端开发
 */
use crate::pb::Spec;
use image::ImageOutputFormat;

mod photon;
pub use photon::Photon;

// Engine trait:未来用于添加更多的engine，主流程只需要替换engine
pub trait Engine {
    // 对engine暗战specs进行一系列有序的处理
    fn apply(&mut self, specs: &[Spec]);
    // 从engine中生成目标图片，这里使用的self不是self的引用
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

// SpecTransform 未来添加更多的spec
pub trait SpecTransform<T> {
    // 对图片使用op做transform
    fn transform(&mut self, op: T);
}
