mod algorithm;
mod constants;
mod error;

use std::{env, fs, process::ExitCode, str::FromStr};

use algorithm::Algorithm;
use constants::{CROSS_ORIGIN_ANONYMOUS, MAX_OF_ARGS};
use error::{SriErrorTypes, SriResult};

fn replace_content_in_file(file_path: &str, target: &str, replacement: &str) -> SriResult<()> {
    // Step 1: Read the file content
    let mut content = fs::read_to_string(file_path)?;

    // Step 2: Replace the content
    content = content.replace(target, replacement);

    // Step 3: Write the modified content back to the file
    fs::write(file_path, &content)?;

    Ok(())
}

fn process_sri(algorithm: &str, airtifact_path: &str) -> SriResult<ExitCode> {
    let algorithm: Algorithm = FromStr::from_str(algorithm)?;
    let paths = fs::read_dir(airtifact_path)?;

    for path in paths {
        let path = path?;

        let file_name = path.file_name();
        let file_name = file_name.as_os_str();
        let file_name = file_name.to_str().ok_or(SriErrorTypes::Unknown)?;

        if file_name == "index.html" {
            let index_file_path = format!("{airtifact_path}/{file_name}");

            let index_html_contents = fs::read_to_string(path.path())?;

            let index_html_contents_without_empty_lines = index_html_contents
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>();

            let links = index_html_contents_without_empty_lines
                .iter()
                .filter(|items| items.starts_with("<link"))
                .collect::<Vec<_>>();

            for link in links {
                let no_close_tag = link.replace("/>", "").replace(">", " ");

                let _ = replace_content_in_file(&index_file_path, &link, &no_close_tag)?;

                let crossorigin = no_close_tag.clone();

                let mut new_crossorigin = crossorigin.clone();

                if crossorigin.contains("crossorigin") {
                    new_crossorigin = crossorigin.replacen("crossorigin", " ", 1)
                }

                new_crossorigin.push_str(CROSS_ORIGIN_ANONYMOUS);

                replace_content_in_file(&index_file_path, &crossorigin, &new_crossorigin)?;

                let mut href_hash = new_crossorigin.clone();

                let href = href_hash
                    .split_whitespace()
                    .filter(|link| link.contains("href"))
                    .map(|maybe_href_key_value| maybe_href_key_value.split_once("="))
                    .map(|maybe_href_key_value| {
                        return if let Some((_, val)) = maybe_href_key_value {
                            val.trim().replacen(r#"""#, "", 2)
                        } else {
                            String::from("Invalid key-val href")
                        };
                    })
                    .collect::<Vec<_>>()
                    .join("");

                let file_contents = fs::read_to_string(format!("{airtifact_path}{href}"))?;
                let integrity_hash = algorithm.integrity_hash(file_contents)?;

                //* GOAL: Add the SRI to the link
                href_hash.push(' ');
                href_hash.push_str(&format!(r#"integrity="{integrity_hash}""#));
                href_hash.push_str(" />");

                replace_content_in_file(&index_file_path, &new_crossorigin, &href_hash)?;
            }

            let scripts = index_html_contents_without_empty_lines
                .iter()
                .filter(|items| items.starts_with("<script"))
                .collect::<Vec<_>>();

            for script in scripts {
                let no_close_tag = script.replacen(">", "", 1).replacen("</script>", " ", 1);

                replace_content_in_file(&index_file_path, &script, &no_close_tag)?;

                let crossorigin = no_close_tag.clone();

                let mut new_crossorigin = crossorigin.clone();

                if crossorigin.contains("crossorigin") {
                    new_crossorigin = crossorigin.replacen("crossorigin", " ", 1)
                }

                new_crossorigin.push_str(CROSS_ORIGIN_ANONYMOUS);

                replace_content_in_file(&index_file_path, &crossorigin, &new_crossorigin)?;

                let mut src_hash = new_crossorigin.clone();

                let src = src_hash
                    .split_whitespace()
                    .filter(|link| link.contains("src"))
                    .map(|maybe_href_key_value| maybe_href_key_value.split_once("="))
                    .map(|maybe_href_key_value| {
                        return if let Some((_, val)) = maybe_href_key_value {
                            val.trim().replacen(r#"""#, "", 2)
                        } else {
                            String::from("Invalid key-val src")
                        };
                    })
                    .collect::<Vec<_>>()
                    .join("");

                let file_contents = fs::read_to_string(format!("{airtifact_path}{src}"))?;
                let integrity_hash = algorithm.integrity_hash(file_contents)?;

                //* GOAL: Add the SRI to the link
                src_hash.push(' ');
                src_hash.push_str(&format!(r#"integrity="{integrity_hash}""#));
                src_hash.push_str("></script>");

                replace_content_in_file(&index_file_path, &new_crossorigin, &src_hash)?;
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}

fn main() -> SriResult<ExitCode> {
    let args = env::args().skip(1).collect::<Vec<String>>();

    let args_len = args.len();

    if args_len == 0 {
        return Err(SriErrorTypes::NoArgs.into());
    }

    if args_len >= 1 && args_len <= MAX_OF_ARGS - 1 {
        return Err(SriErrorTypes::MustContainsTwoArguments.into());
    }

    if args_len == MAX_OF_ARGS {
        let algorithm = &args[0];
        let airtifact_path = &args[1];

        process_sri(&algorithm, &airtifact_path)?;
    }

    if args_len > MAX_OF_ARGS {
        return Err(SriErrorTypes::OverflowArgs(args_len.try_into()?).into());
    }

    print!("Operation completed successfully.");

    Ok(ExitCode::SUCCESS)
}
