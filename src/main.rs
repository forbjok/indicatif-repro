use indicatif::{ProgressBar, ProgressStyle};

const OVERALL_TEMPLATE: &str =
    " {prefix:>8} [{bar:40.cyan/blue}] {bytes}/{total_bytes} @ {bytes_per_sec}, ETA: {eta} {wide_msg:.blue}";

const FILE_TEMPLATE: &str =
    " {prefix:>8} [{bar:40.cyan/blue}] {bytes}/{total_bytes} {wide_msg:.blue}";

const PROGRESS_CHARS: &str = "●●·";

const OVERALL_SIZE: u64 = 1000;
const FILE_SIZE: u64 = 1000;

fn main() {
    let overall_pb = ProgressBar::new(OVERALL_SIZE)
        .with_style(
            ProgressStyle::default_bar()
                .template(OVERALL_TEMPLATE)
                .unwrap()
                .progress_chars(PROGRESS_CHARS),
        )
        .with_prefix("Overall")
        .with_message("Generating checksum set...");

    for i in 0..OVERALL_SIZE {
        let file_pb = ProgressBar::new(FILE_SIZE)
            .with_style(
                ProgressStyle::default_bar()
                    .template(FILE_TEMPLATE)
                    .unwrap()
                    .progress_chars(PROGRESS_CHARS),
            )
            .with_prefix("File")
            .with_message(format!("File {}", i));

        for _ in 0..FILE_SIZE {
            file_pb.inc(1);
        }

        file_pb.finish_and_clear();
        overall_pb.inc(1);
    }

    overall_pb.finish_and_clear();
}
