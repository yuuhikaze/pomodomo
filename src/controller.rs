use std::sync::LazyLock;

use tokio::runtime::Handle;

static ASYNC_RUNTIME_HANDLE: LazyLock<Handle> = LazyLock::new(|| Handle::current());

pub fn get_runtime_handle() -> Handle {
    ASYNC_RUNTIME_HANDLE.clone()
}
