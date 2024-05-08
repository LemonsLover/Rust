use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_fit(&self, rect: &Rect) -> bool {
        self.area() > rect.area()
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn test(self) -> Rect {
        self
    }
}

fn main() {
    let rect: Rect = Rect {
        width: 30,
        height: 50,
    };

    let rect2 = Rect {
        width: 35,
        height: 50,
    };

    let rect3: Rect = Rect {
        width: 60,
        height: 500000,
    };

    let rect4 = Rect {
        width: 1337,
        height: 23213,
    };

    let rect = rect.test();

    let start = Instant::now();
    for _ in 0..1000000 {
        rect.can_fit(&rect2);
        rect.can_fit(&rect3);
        rect.can_fit(&rect4);
        rect4.can_fit(&rect2);
    }
    let end = Instant::now();
    println!("{:?}", end.duration_since(start));

    let start2 = Instant::now();
    for _ in 0..1000000 {
        rect.can_hold(&rect2);
        rect.can_hold(&rect3);
        rect.can_hold(&rect4);
        rect4.can_hold(&rect2);
    }
    let end2 = Instant::now();
    println!("{:?}", end2.duration_since(start2));
}
