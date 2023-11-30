pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Iterator_demonstration() {
        let v1 = vec![1, 3, 4];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), Some(&4));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 3, 4];
        let v1_iter = v1.iter();
        // 调用 sum 方法获取迭代器所有项的总和
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 8);
        // 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权
        // assert_eq!(v1_iter.next(), Some(&1));
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoe_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // 使用 filter 方法和一个捕获 shoe_size 的闭包
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
    // 示例 13-19
    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 12,
                style: String::from("sandal"),
            },
            Shoe {
                size: 14,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoe_in_my_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![Shoe {
                size: 10,
                style: String::from("sneaker")
            }]
        );
    }

    // 让我们创建一个只会从 1 数到 5 的迭代器
    // 示例 13-20 有一个 Counter 结构体定义和一个创建 Counter 实例的关联函数 new：

    #[derive(Debug, PartialEq)]
    struct Counter {
        count: u32,
    }

    // 定义 Counter 结构体和一个创建 count 初值为 0 的 Counter 实例的 new 函数
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    // 在 Counter 结构体上实现 Iterator trait
    // 这里将迭代器的关联类型 Item 设置为 u32，意味着迭代器会返回 u32 值集合
    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
}
