#![warn(missing_docs)]
#![allow(clippy::type_complexity)]
#![doc = include_str!("../README.md")]

mod slice;
pub use slice::{ParallelSlice, ParallelSliceMut};

mod task;
pub use task::Task;

#[cfg(all(feature = "multi-threaded"))]
mod task_pool;
#[cfg(all(feature = "multi-threaded"))]
pub use task_pool::{Scope, TaskPool, TaskPoolBuilder};

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm_worker;

#[cfg(any(not(feature = "multi-threaded")))]
mod single_threaded_task_pool;
#[cfg(any(not(feature = "multi-threaded")))]
pub use single_threaded_task_pool::{Scope, TaskPool, TaskPoolBuilder, ThreadExecutor};

mod usages;
//#[cfg(not(target_arch = "wasm32"))]
pub use usages::tick_global_task_pools_on_main_thread;
pub use usages::{AsyncComputeTaskPool, ComputeTaskPool, IoTaskPool};

#[cfg(all(feature = "multi-threaded"))]
mod thread_executor;
#[cfg(all(feature = "multi-threaded"))]
pub use thread_executor::{ThreadExecutor, ThreadExecutorTicker};

mod iter;
pub use iter::ParallelIterator;

#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        iter::ParallelIterator,
        slice::{ParallelSlice, ParallelSliceMut},
        usages::{AsyncComputeTaskPool, ComputeTaskPool, IoTaskPool},
    };
}

use std::num::NonZeroUsize;

/// Gets the logical CPU core count available to the current process.
///
/// This is identical to [`std::thread::available_parallelism`], except
/// it will return a default value of 1 if it internally errors out.
///
/// This will always return at least 1.
pub fn available_parallelism() -> usize {
    #[cfg(not(target_arch = "wasm32"))]
    return std::thread::available_parallelism()
        .map(NonZeroUsize::get)
        .unwrap_or(1);
    #[cfg(target_arch = "wasm32")]
    return web_sys::window()
        .expect("Missing Window")
        .navigator()
        .hardware_concurrency() as _;
}
