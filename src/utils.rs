pub(super) fn format_bytes(bytesval: usize, precision: usize) -> (String, String, String) {
    let mut val = bytesval as f32;

    for unit in ["bytes", "KiB", "MiB", "GiB", "TiB"] {
        if val < 1024.0 {
            return (
                format!("{:.precision$}", val, precision = precision),
                unit.to_owned(),
                format!("{:.precision$} {}", val, unit, precision = precision),
            );
        }

        val /= 1024.0;
    }

    (
        format!("{:.precision$}", bytesval, precision = precision),
        "".to_owned(),
        format!("{:.precision$}", bytesval, precision = precision),
    )
}

pub(super) fn format_download_bytes(downloaded: usize, total: usize) -> String {
    let downloaded = format_bytes(downloaded, 2);
    let mut total = format_bytes(total, 2);

    if total.1 == "MiB" {
        total.0 = total.0.split('.').next().unwrap().to_owned();
    }

    if downloaded.1 == total.1 {
        format!("{} / {} {}", downloaded.0, total.0, downloaded.1)
    } else {
        format!("{} / {}", downloaded.2, total.2)
    }
}

// pub(super) fn find_hls_dash_links(text: &str) -> Vec<String> {
//     let re = regex::Regex::new(r"(https|ftp|http)://([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&:/~+#-]*[\w@?^=%&/~+#-]\.(m3u8|m3u|mpd))").unwrap();
//     let links = re
//         .captures_iter(text)
//         .map(|caps| caps.get(0).unwrap().as_str().to_string())
//         .collect::<Vec<String>>();

//     let mut unique_links = vec![];
//     for link in links {
//         if !unique_links.contains(&link) {
//             unique_links.push(link);
//         }
//     }
//     unique_links
// }


// pub(super) fn scrape_website_message(url: &str) -> String {
//     format!(
//         "No links found on website source.\n\n\
//         {} Consider using {} subcommand and then \
//         run the {} subcommand with same arguments by replacing the {} with captured url.\n\n\
//         Suppose first command captures https://streaming.site/video_001/master.m3u8\n\
//         $ vsd capture {}\n\
//         $ vsd save https://streaming.site/video_001/master.m3u8 \n\n\
//         {} Consider using {} subcommand \
//         and then run {} subcommand with saved playlist file as {}. \n\n\
//         Suppose first command saves master.m3u8\n\
//         $ vsd collect --build {}\n\
//         $ vsd save master.m3u8",
//         "TRY THIS:".colorize("yellow"),
//         "capture".colorize("bold green"),
//         "save".colorize("bold green"),
//         "INPUT".colorize("bold green"),
//         url,
//         "OR THIS:".colorize("yellow"),
//         "collect".colorize("bold green"),
//         "save".colorize("bold green"),
//         "INPUT".colorize("bold green"),
//         url,
//     )
// }
