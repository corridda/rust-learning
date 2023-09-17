//! 19.4. Advanced Functions and Closures

/// Function Pointers
mod function_pointers {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn multiply_by_three(x: i32) -> i32 {
        3 * x
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    pub fn examine_function_pointers() {
        println!("do_twice(add_one, 2): {}", do_twice(add_one, 5));
        println!("do_twice(multiply_by_three, 2): {}", do_twice(multiply_by_three, 5));

        let list_of_statuses: Vec<Status> = (0u32..25).map(Status::Value).collect();
        println!("list_of_statuses: {list_of_statuses:?}\n");
    }
}

mod returning_closures {
    use std::ops::Deref;

    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    pub fn explore_returning_closers() {
        let my_closure_value = returns_closure();
        println!("my_closure.deref()(10): {}", my_closure_value.deref()(10))
    }

}

fn main() {
    use function_pointers;
    use returning_closures;

    function_pointers::examine_function_pointers();
    returning_closures::explore_returning_closers();
}
