
# Usage

```rust

use sync_async::sync_async;

#[sync_async] 
pub mod utils {

    #[maybe_async]
    pub fn run_blocking<T>(task: impl FnOnce() -> T) -> T {
        #[if_async] 
        let t = todo!();
    
        #[if_sync]
        let t = task();

        t
    }
}

#[sync_async(
    use std::sync::Arc;
    use std::io::Read as Reader,
)]
pub struct Foo<R: Reader> {
    reader: Arc<R>
}


#[sync_async(
    use std::io::Read as Reader;
    use(if_sync) sync_utils::run_blocking,
    use(if_async) async_utils::run_blocking,
)]
impl Foo<std::fs::File> {

    /// [Reader]
    #[maybe_async]
    pub fn read(&self) -> std::io::Result<Vec<u8>> {
        let mut reader = std::sync::Arc::clone(&self.reader);
        
        run_blocking(move || {
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            Ok(buf)
        }).await
    }

    /// [Reader]
    #[always_sync]
    pub fn read_sync(&self) -> std::io::Result<Vec<u8>> {
        let mut reader = std::sync::Arc::clone(&self.reader);
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

```

It is converted into the following.

```rust

pub mod async_utils {

    pub async fn run_blocking<T>(task: impl FnOnce() -> T) -> T {
        let t = todo!();
        t
    }
}
pub mod sync_utils {

    pub fn run_blocking<T>(task: impl FnOnce() -> T) -> T {
        let t = task();
        t
    }
}

pub struct AsyncFoo<R: std::io::Read> {
    reader: std::sync::Arc<R>,
}
pub struct SyncFoo<R: std::io::Read> {
    reader: std::sync::Arc<R>,
}

impl AsyncFoo<std::fs::File> {

    /// [Reader](std::io::Read)
    pub async fn read(&self) -> std::io::Result<Vec<u8>> {
        use async_utils::run_blocking;
        use std::io::Read as Reader;
        use AsyncFoo as Foo;

        let mut reader = std::sync::Arc::clone(&self.reader);
        run_blocking(move || {
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            Ok(buf)
        })
        .await
    }

    /// [Reader](std::io::Read)
    pub fn read_sync(&self) -> std::io::Result<Vec<u8>> {
        use async_utils::run_blocking;
        use std::io::Read as Reader;
        use AsyncFoo as Foo;

        let mut reader = std::sync::Arc::clone(&self.reader);
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(buf)
    }
}
impl SyncFoo<std::fs::File> {

    /// [Reader](std::io::Read)
    pub fn read(&self) -> std::io::Result<Vec<u8>> {
        use std::io::Read as Reader;
        use sync_utils::run_blocking;
        use SyncFoo as Foo;

        let mut reader = std::sync::Arc::clone(&self.reader);
        run_blocking(move || {
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            Ok(buf)
        })
    }

    /// [Reader](std::io::Read)
    pub fn read_sync(&self) -> std::io::Result<Vec<u8>> {
        use std::io::Read as Reader;
        use sync_utils::run_blocking;
        use SyncFoo as Foo;

        let mut reader = std::sync::Arc::clone(&self.reader);
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

```