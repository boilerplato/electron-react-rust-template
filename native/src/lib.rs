use neon::register_module;

pub use error::Error;

mod error;
mod ext;
mod helpers;
mod prelude;
mod runtime;

pub type Result<T> = std::result::Result<T, crate::Error>;

register_module!(mut cx, {
    cx.export_function("nativeStartTokioRuntime", runtime::start_runtime)
        .unwrap();
    cx.export_function("nativeShutdownTokioRuntime", runtime::shutdown_runtime)
        .unwrap();

    Ok(())
});
