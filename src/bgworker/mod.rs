pub mod normal;
pub mod upgrade;

pub unsafe fn init() {
    use pgrx::bgworkers::BackgroundWorkerBuilder;
    use pgrx::bgworkers::BgWorkerStartTime;
    use std::time::Duration;
    BackgroundWorkerBuilder::new("vectors")
        .set_function("vectors_main")
        .set_library("vectors")
        .set_argument(None)
        .enable_shmem_access(None)
        .set_start_time(BgWorkerStartTime::PostmasterStart)
        .set_restart_time(Some(Duration::from_secs(1)))
        .load();
}

#[no_mangle]
extern "C" fn vectors_main(_arg: pgrx::pg_sys::Datum) {
    let _ = std::panic::catch_unwind(crate::bgworker::main);
}

pub fn main() {
    pub struct AllocErrorPanicPayload {
        pub layout: std::alloc::Layout,
    }
    {
        let mut builder = env_logger::builder();
        builder.target(env_logger::Target::Stderr);
        #[cfg(not(debug_assertions))]
        {
            builder.filter(None, log::LevelFilter::Info);
        }
        #[cfg(debug_assertions)]
        {
            builder.filter(None, log::LevelFilter::Trace);
        }
        builder.init();
    }
    std::panic::set_hook(Box::new(|info| {
        if let Some(oom) = info.payload().downcast_ref::<AllocErrorPanicPayload>() {
            log::error!("Out of memory. Layout: {:?}.", oom.layout);
            return;
        }
        let backtrace;
        #[cfg(not(debug_assertions))]
        {
            backtrace = std::backtrace::Backtrace::capture();
        }
        #[cfg(debug_assertions)]
        {
            backtrace = std::backtrace::Backtrace::force_capture();
        }
        log::error!("Panickied. Info: {:?}. Backtrace: {}.", info, backtrace);
    }));
    std::alloc::set_alloc_error_hook(|layout| {
        std::panic::panic_any(AllocErrorPanicPayload { layout });
    });
    use service::worker::Worker;
    use std::path::Path;
    let path = Path::new("pg_vectors");
    if path.try_exists().unwrap() {
        if Worker::check(path.to_owned()) {
            let worker = Worker::open(path.to_owned());
            self::normal::normal(worker);
        } else {
            self::upgrade::upgrade();
        }
    } else {
        let worker = Worker::create(path.to_owned());
        self::normal::normal(worker);
    }
}
