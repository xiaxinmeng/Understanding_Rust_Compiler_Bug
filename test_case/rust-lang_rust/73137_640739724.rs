rust
struct DD {
    a: String,
}

// DD is dropped twice
impl Drop for DD {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

struct Yolo {
    a: DD,
    b: Option<DD>,
}

async fn gg(x: &Yolo) {}

fn main() {
    futures::executor::block_on(async {
        let d2 = async { DD { a: "a".to_owned() } };

        // Swap the order here and it works
        let action = Yolo {
            b: None,
            a: d2.await,
        };
        gg(&action).await;
    });
}

