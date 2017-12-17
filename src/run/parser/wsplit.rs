//! Provides the parser for WSplit splits files.

use std::io::{self, BufRead};
use std::result::Result as StdResult;
use std::num::{ParseFloatError, ParseIntError};
use {Image, Run, Segment, Time, TimeSpan};

quick_error! {
    /// The Error type for splits files that couldn't be parsed by the WSplit
    /// Parser.
    #[derive(Debug)]
    pub enum Error {
        /// Expected the name of the segment, but didn't find it.
        ExpectedSegmentName {}
        /// Expected the old time, but didn't find it.
        ExpectedOldTime {}
        /// Expected the split time, but didn't find it.
        ExpectedPbTime {}
        /// Expected the best segment time, but didn't find it.
        ExpectedBestTime {}
        /// Failed to parse the amount of attempts.
        Attempt(err: ParseIntError) {
            from()
        }
        /// Failed to parse a time.
        Time(err: ParseFloatError) {
            from()
        }
        /// Failed to read from the source.
        Io(err: io::Error) {
            from()
        }
    }
}

/// The Result type for the WSplit Parser.
pub type Result<T> = StdResult<T, Error>;

/// Attempts to parse a WSplit splits file. In addition to the source to parse,
/// you need to specify if additional files for the icons should be loaded from
/// the file system. If you are using livesplit-core in a server-like
/// environment, set this to `false`. Only client-side applications should set
/// this to `true`.
pub fn parse<R: BufRead>(source: R, load_icons: bool) -> Result<Run> {
    let mut run = Run::new();
    let mut icon_buf = Vec::new();
    let mut icons_list = Vec::new();
    let mut old_run_exists = false;

    for line in source.lines() {
        let line = line?;
        if !line.is_empty() {
            if line.starts_with("Title=") {
                run.set_category_name(&line["Title=".len()..]);
            } else if line.starts_with("Attempts=") {
                run.set_attempt_count(line["Attempts=".len()..].parse()?);
            } else if line.starts_with("Offset=") {
                let offset = &line["Offset=".len()..];
                if !offset.is_empty() {
                    run.set_offset(TimeSpan::from_milliseconds(-offset.parse::<f64>()?));
                }
            } else if line.starts_with("Size=") {
                // Ignore
            } else if line.starts_with("Icons=") {
                if load_icons {
                    let icons = &line["Icons=".len()..];
                    icons_list.clear();
                    for path in icons.split(',') {
                        if path.len() >= 2 {
                            let path = &path[1..path.len() - 1];
                            if let Ok(image) = Image::from_file(path, &mut icon_buf) {
                                icons_list.push(image);
                                continue;
                            }
                        }
                        icons_list.push(Image::default());
                    }
                }
            } else {
                // must be a split Kappa
                let mut split_info = line.split(',');

                let segment_name = split_info.next().ok_or(Error::ExpectedSegmentName)?;
                let old_time = split_info.next().ok_or(Error::ExpectedOldTime)?;
                let pb_time = split_info.next().ok_or(Error::ExpectedPbTime)?;
                let best_time = split_info.next().ok_or(Error::ExpectedBestTime)?;

                let mut segment = Segment::new(segment_name);
                let pb_real_time = TimeSpan::from_seconds(pb_time.parse()?);
                let best_real_time = TimeSpan::from_seconds(best_time.parse()?);
                let old_real_time = TimeSpan::from_seconds(old_time.parse()?);

                let mut pb_time = Time::new();
                let mut best_time = Time::new();
                let mut old_time = Time::new();

                if pb_real_time != TimeSpan::zero() {
                    pb_time.real_time = Some(pb_real_time);
                }
                if best_real_time != TimeSpan::zero() {
                    best_time.real_time = Some(best_real_time);
                }
                if old_real_time != TimeSpan::zero() {
                    old_time.real_time = Some(old_real_time);
                    *segment.comparison_mut("Old Run") = old_time;
                    old_run_exists = true;
                }

                segment.set_personal_best_split_time(pb_time);
                segment.set_best_segment_time(best_time);
                run.push_segment(segment);
            }
        }
    }

    if old_run_exists {
        run.add_custom_comparison("Old Run");
    }

    for (icon, segment) in icons_list.into_iter().zip(run.segments_mut().iter_mut()) {
        segment.set_icon(icon);
    }

    Ok(run)
}
