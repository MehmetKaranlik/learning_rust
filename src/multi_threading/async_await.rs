#![allow(dead_code)]

/// The `async` keyword is used to define an asynchronous function.
/// It goes well with Tokio, but it can also be used with other asynchronous runtimes.
/// 'async' returns a future, which is a type that represents an asynchronous computation.
/// The `await` keyword is used to wait for a future to complete.
/// It can only be used inside an `async` function.

async fn async_function(x: &u32) -> u32 {
    return x * 2;
}


async fn consumer() {
    let future = async_function(&2); // Type is `impl Future<Output = u32>`
    println!("Future: {:?}", future.await);
}

// or

async fn consumer2() {
    let future = async_function(&2); // Type is `impl Future<Output = u32>`
    drop(future); // No need to await the future if we don't care about the result
}

// or
async fn tokio_tasks() -> u32 {
    let future = async_function(&2);
    return tokio::spawn(future).await.unwrap(); // Spawn the future on the Tokio runtime
}