use indicatif::{ProgressBar, ProgressStyle};

pub fn exec_sync_with_spinner<F>(stage_msg: &'static str, execor: F)
where
    F: FnOnce() -> (),
{
    let progress_bar = ProgressBar::new_spinner();

    progress_bar.set_style(ProgressStyle::default_spinner());
    progress_bar.enable_steady_tick(200);
    progress_bar.set_message(stage_msg);

    execor();

    progress_bar.finish_with_message(format!("✅{}", stage_msg))
}
