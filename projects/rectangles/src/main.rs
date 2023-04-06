#[derive(Debug)]
struct Rectangle { width: u64, height: u64 }

impl Rectangle {
    fn square(size: u64) -> Self {
        Self { width: size, height: size }
    }
    fn area(&self) -> u64 { self.width * self.height }
    fn clear(&mut self) {
        self.width = 0;
        self.height = 0;
    }
    fn fit_in(&self, rect: &Rectangle) -> bool {
        self.width <= rect.width && self.height <= rect.height
    }
}

fn main() {
    {
        let width = 30;
        let height = 50;
        let area = rect_area(width, height);
        println!("Area of {} x {} rectangle is {}", width, height, area);
    } {
        let dimensions = (30, 50);
        let area = rect_area_tuple(dimensions);
        println!("Area of {} x {} rectangle is {}", dimensions.0, dimensions.1, area);
    } {
        let rectangle = Rectangle { width: 30, height: 50 };
        println!("Rectangle: {:#?}", rectangle);
        let area = rect_area_struct(&rectangle);
        println!("Area of {} x {} rectangle is {}", rectangle.width, rectangle.height, area);
    } {
        let x = 2;
        let rectangle = Rectangle {
            width: dbg!(15 * x),
            height: 50
        };
        dbg!(&rectangle);
        let area = dbg!(rect_area_struct(&rectangle));
        println!("Area of {} x {} rectangle is {}", rectangle.width, rectangle.height, area);
    } {
        let rectangle = Rectangle { width: 30, height: 50 };
        let area = rectangle.area();
        println!("Area of {} x {} rectangle is {}", rectangle.width, rectangle.height, area);
    } {
        let rectangle = Rectangle { width: 30, height: 50 };
        let area = Rectangle::area(&rectangle);
        println!("Area of {} x {} rectangle is {}", rectangle.width, rectangle.height, area);
    } {
        let mut rectangle = Rectangle { width: 30, height: 50 };
        println!("{:?}", rectangle);
        rectangle.clear();
        println!("{:?}", rectangle);
    } {
        let rect1 = Rectangle { width: 100, height: 100 };
        let rect2 = Rectangle { width: 50, height: 50 };
        let rect3 = Rectangle { width: 50, height: 120 };
        println!("rect 2 fit in rect 1: {}", rect2.fit_in(&rect1));
        println!("rect 3 fit in rect 1: {}", rect3.fit_in(&rect1));
    } {
        let sq = Rectangle::square(5);
        let area = Rectangle::area(&sq);
        println!("Area of {} x {} rectangle is {}", sq.width, sq.height, area);
    }
}

fn rect_area(width: u64, height: u64) -> u64 {
    width * height
}

fn rect_area_tuple(dimensions: (u64, u64)) -> u64 {
    dimensions.0 * dimensions.1
}

fn rect_area_struct(rectangle: &Rectangle) -> u64 {
    rectangle.width * rectangle.height
}
