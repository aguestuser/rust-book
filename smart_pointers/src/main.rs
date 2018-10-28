use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

fn main() {
    /*

    Variants of smart pointers:

    * Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
    * Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
    * Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

     */

    {
        /*******************
         * RECURSIVE TYPES
         *******************/
        enum List<T> {
            Cons(T, Box<List<T>>),
            Nil,
        }

        // box pointer
        let b = Box::new(5);
        println!("b = {}", b);

        // using box for rfecursive types
        let _list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );
    }

    {
        /*******************
         * DEREFERENCING
         *******************/

        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }

        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);

        // deref coercion
        // deref coercion
        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }

        let m = MyBox::new(String::from("Rust"));
        hello(&m);
        //instead of
        hello(&(*m)[..]);
    }

    {
        /****************************************
         * Implementing Drop (for cleanup code)
         ****************************************/

        struct CustomSmartPointer {
            data: String,
        }

        impl Drop for CustomSmartPointer {
            fn drop(&mut self) {
                println!("Dropping CustomSmartPointer with data `{}`!", self.data)
            }
        }

        let csp = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _csp1 = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointer created.");

        drop(csp); // manually drop c before it goes out of scope
        println!("CustomSmartPointer dropped before the end of scope.");
    }

    {
        /*****************************************************
         * Reference counting for immutable structure sharing
         *****************************************************/

        // reference counting
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

        let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
        println!(">>>> immutable structure sharing <<<<");
        println!("count after creating a = {}", Rc::strong_count(&a));
        let _b = List::Cons(5, Rc::clone(&a)); // DO: increment ref count
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let _c = List::Cons(5, Rc::clone(&a)); // DON'T deep copy list (yay!)
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope= {}", Rc::strong_count(&a));
    }

    {
        /*****************************************************
         * mutable shared list item (ref cell inside of rc)
         *****************************************************/

        #[derive(Debug)]
        enum List<T> {
            Cons(Rc<RefCell<T>>, Rc<List<T>>),
            Nil,
        }

        // rc + refcell for shared mutable list
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

        let b = List::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = List::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!(">>>> internally mutable shared list item <<<<");
        println!("a after: {:?}", a);
        println!("b after: {:?}", b);
        println!("c after: {:?}", c);
    }

    {
        /********************************************************
         * memory leaks due to ref cycles (rc inside of ref cell)
         ********************************************************/

        #[derive(Debug)]
        enum List<T> {
            Cons(T, RefCell<Rc<List<T>>>),
            Nil,
        }

        impl<T> List<T> {
            fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
                match self {
                    List::Cons(_, tl) => Some(tl),
                    List::Nil => None,
                }
            }
        }

        println!(">>>>> mutable structure sharing <<<<<<");
        let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
        println!("a inital rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            // change pointer to a's tail
            // creating a cycle (b/c b points to a)
            *link.borrow_mut() = Rc::clone(&b)
        }
        println!("a rc count after changing a = {}", Rc::strong_count(&a));
        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        // ^--- this would create a memory leak
        // b/c of cycle, a & b's ref count will never go to 0
        // when b is droped, a still has a reference to it
        // (:. both a & b counts will go to 1, not 0)
        // so a & b's  heap memory will never be deallocated

        // this would overflow the stack b/c of cycle:
        // println!("a next item = {:?}", a.tail());
    }

    {
        /******************************************
         * preventing reference cycles with Weak<T>
         ******************************************/

        /*
        When you call Rc::downgrade, you get a smart pointer of type Weak<T>. Instead of increasing the strong_count in the Rc<T> instance by 1, calling Rc::downgrade increases the weak_count by 1. The Rc<T> type uses weak_count to keep track of how many Weak<T> references exist, similar to strong_count. The difference is the weak_count doesn’t need to be 0 for the Rc<T> instance to be cleaned up.

        Strong references are how you can share ownership of an Rc<T> instance. Weak references don’t express an ownership relationship. They won’t cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0.

        Because the value that Weak<T> references might have been dropped, to do anything with the value that a Weak<T> is pointing to, you must make sure the value still exists. Do this by calling the upgrade method on a Weak<T> instance, which will return an Option<Rc<T>>. You’ll get a result of Some if the Rc<T> value has not been dropped yet and a result of None if the Rc<T> value has been dropped. Because upgrade returns an Option<T>, Rust will ensure that the Some case and the None case are handled, and there won’t be an invalid pointer.

        As an example, rather than using a list whose items know only about the next item, we’ll create a tree whose items know about their children items and their parent items.
         */

        println!(">>> preventing reference cycles with weak refs <<<");

        #[derive(Debug)]
        struct Node<T> {
            value: T,
            parent: RefCell<Weak<Node<T>>>,
            children: RefCell<Vec<Rc<Node<T>>>>,
        }

        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "PRE-ASSIGNMENT: leaf parent = {:?}",
            leaf.parent.borrow().upgrade()
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            // give leaf a weak pointer to its parent
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
            println!(
                "POST-ASSIGNMENT: leaf parent = {:?}",
                leaf.parent.borrow().upgrade()
            );

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!(
            "POST-SCOPE: leaf parent = {:?}",
            leaf.parent.borrow().upgrade()
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    // watch out for memory leaks!!!
} // implementations of `Drop#drop` will be called  here
