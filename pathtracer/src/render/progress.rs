use indicatif::ProgressBar;

pub fn get_progressbar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_draw_delta((total / 10000).max(1));
    pb.set_style(indicatif::ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent:>3}%: {pos}/{len} pixels (ETA: {eta})",
    ));
    pb
}
