//! Indicator is a lib which will allow you to manipulate what you show to the user.

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;

static STYLE: Lazy<ProgressStyle> = Lazy::new(|| {
    ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} {msg}",
    )
    .expect("can't fail")
    .progress_chars("##-")
});

#[derive(Debug, Clone)]
pub struct Indicator {
    pub bars: MultiProgress,
}

impl Indicator {
    /// Create a new `Indicator` stack.
    pub fn new() -> Self {
        Self {
            bars: MultiProgress::new(),
        }
    }

    pub fn follow(&self, progress_bar: ProgressBar) -> ProgressBar {
        let pb = self.bars.add(progress_bar);
        pb.set_style(STYLE.to_owned());
        pb
    }

    pub fn println<I: AsRef<str>>(&self, msg: I) -> std::io::Result<()> {
        self.bars.println(msg)
    }
}
