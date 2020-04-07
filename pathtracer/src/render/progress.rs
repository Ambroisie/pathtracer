use indicatif::{ProgressBar, ProgressStyle};

pub fn get_progressbar(total: u64, style: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_draw_delta((total / 10000).max(1));
    pb.set_style(ProgressStyle::default_bar().template(style));
    pb
}

pub fn get_pixels_progressbar(total: u64) -> ProgressBar {
    get_progressbar(
        total,
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent:>3}%: {pos}/{len} pixels (ETA: {eta})",
    )
}

pub fn get_passes_progressbar(total: u32) -> ProgressBar {
    let pb = get_progressbar(
        total as u64,
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent:>3}%: {pos}/{len} passes (ETA: {eta})",
    );

    pb.enable_steady_tick(1000);

    pb
}
