fn main() {
    let nums1 = vec![34, 50, 25, 100, 65];
    let nums2 = vec![102.0, 34.1, 6000.2, 89.111, 54.3];

    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = &item
            }
        }
        &largest
    }

    assert_eq!(largest(&nums1), &100);
    assert_eq!(largest(&nums2), &6000.2);
    assert_eq!(largest(&nums1[..2]), &50);

    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        // generic implementation
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32, f32> {
        // impl for a concrete type
        fn dist_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    impl<T, U> Point<T, U> {
        // methods can use different types than those paramaterized in struct defn
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    // bonus: rust generics are very fast b/c they leverage monomorphization
    // ie: they degericize generic types based on which paramaterized types are actually used
    // given Option<T> and instances of Option<i32> and Option<String>, compiler would
    // rewrite to Option_i32::some(1), Option_String::Some("hi"), etc..
}
