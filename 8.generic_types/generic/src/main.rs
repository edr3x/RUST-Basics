fn main() {
    let num_list: Vec<i32> = vec![44, 54, 435, 33];

    let largest = get_largest(num_list);

    println!("The largest number is {}", largest);

    let char_list: Vec<char> = vec!['y', 'm', 'b', 'd'];

    let largest = get_largest(char_list);

    println!("The largest number is {}", largest);

    println!("\n");

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    //let p3 = Point { x: 3, y: 4.5 };

    g_struct();
}

//* Generic function */
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest: T = list[0];
    for num in list {
        if num > largest {
            largest = num;
        }
    }
    largest
}

//* Generic struct */
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn g_struct() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x ={}, p3.y = {} ", p3.x, p3.y);
}
