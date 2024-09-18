use std::cell::Cell;

fn main() {
    Notifier::new()
        .print("Hello from literal string")
        .print(BytesAdapter {
            bytes: Bytes {
                inner: "Hello from Bytes".as_bytes().to_vec(),
            },
        });
}

trait ChainNotify {
    fn print(&self, msg: impl AsText) -> &Self;
}

struct Notifier {
    count: Cell<usize>,
}

impl Notifier {
    fn new() -> Self {
        Self {
            count: Cell::new(0),
        }
    }
}

impl ChainNotify for Notifier {
    fn print(&self, msg: impl AsText) -> &Self {
        let count = self.count.get();
        println!("{}: {}", self.count.replace(count + 1), msg.text());
        self
    }
}

trait AsText {
    fn text(&self) -> String;
}

impl AsText for &str {
    fn text(&self) -> String {
        self.to_string()
    }
}

struct Bytes {
    inner: Vec<u8>,
}

struct BytesAdapter {
    bytes: Bytes,
}

impl AsText for BytesAdapter {
    fn text(&self) -> String {
        String::from_utf8_lossy(&self.bytes.inner).to_string()
    }
}
