rust
#[async_trait]
pub trait Store {
    async fn fetch_pod_modules(&self, pod: &Pod) -> anyhow::Result<HashMap<String, Vec<u8>>> {
        // ...
    }
}

// Error:
// the trait `kubelet::store::Store` cannot be made into an object
// the trait cannot be made into an object because method `fetch_pod_modules`
// references the `Self` type in its `where` clause
