/*
 * @Description:
 * @Author: 李昶
 * @Date: 2022-10-25 22:56:36
 * @LastEditors: 李昶
 * @LastEditTime: 2022-10-26 00:07:31
 * @Profile: 一个比较废柴的前端开发
 */

mod dialect;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
