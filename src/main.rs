fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    #[test]
    fn test_spawn_thread() {
        thread::spawn(|| {
            for i in 1..=5 {
                println!("{}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::sleep(Duration::from_secs(6));
    }

    #[test]
    fn test_join_thread() {
        let handle = thread::spawn(|| {
            let mut counter = 0;
            for i in 1..=5 {
                println!("{}", i);
                thread::sleep(Duration::from_secs(1));
                counter += 1;
            }

            return counter;
        });

        println!("Waiting handle");

        let result = handle.join();
        match result {
            Ok(counter) => println!("Result {}", counter),
            Err(e) => println!("{:?}", e),
        }
    }

    fn calculate() -> i32 {
        let mut counter = 0;
        let current = thread::current();

        for i in 1..=5 {
            match current.name() {
                None => {
                    println!("{:?} : Counter : {}", current.id(), i);   
                }
                Some(name) => {
                    println!("{} : Counter : {}", name, i);
                }
            }
            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }

        return counter;
    }

    #[test]
    fn test_sequential() {
        let result1 = calculate();
        let result2 = calculate();
        println!("Total Counter 1 : {}", result1);
        println!("Total Counter 2 : {}", result2);
        println!("Application Counter");
    }

    #[test]
    fn test_parallel() {
        let handle1 = thread::spawn(|| calculate());
        let handle2 = thread::spawn(|| calculate());

        let result1 = handle1.join();
        let result2 = handle2.join();

        match result1 {
            Ok(counter) => { println!("Total Counter 1 : {}", counter) }
            Err(error) => { println!("Error : {:?}", error) }
        }

        match result2 {
            Ok(counter) => { println!("Total Counter 2 : {}", counter) }
            Err(error) => { println!("Error : {:?}", error) }
        }

        println!("Application Finish");
    }

    #[test]
    fn test_closure() {
        let current_thread = thread::current();
        println!("{}", current_thread.name().unwrap());

        let name = String::from("Ricid");
        let closure = move || {
            thread::sleep(Duration::from_secs(2));
            println!("Hello {}", name);
        };

        let handle = thread::spawn(closure);
        handle.join().unwrap();
    }

    #[test]
    fn test_thread_factory() {
        let factory = thread::Builder::new().name("My Thread".to_string());
        let handle = factory.spawn(calculate).expect("Failed to create a new thread");
        let total = handle.join().unwrap();

        println!("Total Counter : {}", total);
    }
}

