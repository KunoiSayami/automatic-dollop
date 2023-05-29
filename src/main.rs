
#![windows_subsystem = "windows"]
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use rodio::{OutputStream, Sink, Source};
use rodio::source::SineWave;


fn main() -> anyhow::Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let source = SineWave::new(440.0).repeat_infinite().amplify(0.0);
    sink.append(source);


    let b  = Arc::new(AtomicBool::new(false));

    let new_b = b.clone();


    ctrlc::set_handler(move || {
        new_b.store(true, Ordering::Relaxed);
    })?;

    loop {
        if b.load(Ordering::Relaxed) {
            break
        }
        std::thread::sleep(Duration::from_millis(100));
    }


    Ok(())
}
