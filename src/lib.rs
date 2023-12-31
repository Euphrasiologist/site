use anyhow::{bail, Result};
use chrono::NaiveDate;
use std::path::Path;

/// The HTML template used for every page on the website.
mod template;

pub enum WebpageType {
    Index,
    Blog,
}

/// Build the website, copying all relevant files from other directories into
/// the `public` folder.
pub fn build_website(content_dir: &str, output_dir: &str) -> Result<()> {
    // remove everything except about
    let _ = std::fs::remove_dir_all(output_dir);

    // copy over the CV (and any other static docs)
    std::fs::create_dir_all("./public/cv")?;
    let copy_status = std::process::Command::new("cp")
        .arg("-r")
        .arg("./cv/output.pdf")
        .arg("public/cv/")
        .output()?;

    eprintln!(
        "Err: {}\nOut: {}",
        std::str::from_utf8(&copy_status.stderr)?,
        std::str::from_utf8(&copy_status.stdout)?
    );

    // copy over htmls
    std::fs::create_dir_all("./public/external_htmls")?;
    let copy_status_htmls = std::process::Command::new("cp")
        .arg("-r")
        .arg("./external_htmls/")
        .arg("public/external_htmls/")
        .output()?;

    eprintln!(
        "Err: {}\nOut: {}",
        std::str::from_utf8(&copy_status_htmls.stderr)?,
        std::str::from_utf8(&copy_status_htmls.stdout)?
    );

    // copy over images
    std::fs::create_dir_all("./public/img")?;
    let copy_status_img = std::process::Command::new("cp")
        .arg("-r")
        .arg("./img/")
        .arg("public")
        .output()?;

    eprintln!(
        "Err: {}\nOut: {}",
        std::str::from_utf8(&copy_status_img.stderr)?,
        std::str::from_utf8(&copy_status_img.stdout)?
    );

    // process the markdown files
    let markdown_files: Vec<String> = walkdir::WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().display().to_string().ends_with(".md"))
        .map(|e| e.path().display().to_string())
        .collect();

    let mut html_files = Vec::with_capacity(markdown_files.len());

    for file in &markdown_files {
        let mut html = template::HEADER.to_owned();

        let markdown_file = std::fs::read_to_string(file)?;

        let parser =
            pulldown_cmark::Parser::new_ext(&markdown_file, pulldown_cmark::Options::all());

        let mut body = String::new();

        pulldown_cmark::html::push_html(&mut body, parser);

        html.push_str(template::render_body(&body, WebpageType::Blog).as_str());
        html.push_str(template::FOOTER);

        let html_file = file
            .replace(content_dir, output_dir)
            .replace(".md", ".html");

        let folder = Path::new(&html_file).parent().unwrap();

        std::fs::create_dir_all(folder)?;

        std::fs::write(&html_file, html)?;
        html_files.push(html_file);
    }

    write_index(html_files, output_dir)?;

    Ok(())
}

#[derive(Debug)]
struct BlogPost {
    inner_html: String,
    date: NaiveDate,
}

/// Write the index, entry point for the website.
fn write_index(files: Vec<String>, output_dir: &str) -> Result<()> {
    let mut html = template::HEADER.to_owned();

    let all_blogs = files
        .into_iter()
        .map(|file| {
            let file = file.trim_start_matches(output_dir);
            let title = file.trim_start_matches('/').trim_end_matches(".html");

            // split the date off and put to the side?
            let date: Vec<_> = title.clone().split('-').collect();
            let date_op = match date.get(1) {
                Some(d) => d.to_owned().trim(),
                None => "01.01.1900", // some very early date?
            };

            let chrono_date = match NaiveDate::parse_from_str(date_op, "%d.%m.%Y") {
                Ok(d) => d,
                Err(err) => bail!("There was an error parsing one of the dates: {}", err),
            };

            Ok(BlogPost {
                inner_html: format!(
                    r#"<div class="blog_link">
                <div id="blog_link"><a href="{}" id="blog_link">{}</a></div>
                <div class="blog_date"><span class="blog_date">{}</span></div>
                </div>
                "#,
                    file,
                    // so we are not repeating the info
                    title.replace(&format!(" - {}", date_op), ""),
                    match date_op {
                        "01.01.1900" => "".to_string(),
                        d => d.to_string(),
                    }
                ),
                date: chrono_date,
            })
        })
        .collect::<Result<Vec<BlogPost>>>();

    // order blogs by date ascending
    let mut all_blogs = all_blogs?;
    all_blogs.sort_by(
        |BlogPost {
             inner_html: _,
             date: d1,
         },
         BlogPost {
             inner_html: _,
             date: d2,
         }| d2.cmp(d1),
    );

    let body = all_blogs
        .iter()
        .map(
            |BlogPost {
                 inner_html,
                 date: _,
             }| inner_html.clone(),
        )
        .collect::<Vec<String>>()
        .join("<br/>\n");

    html.push_str(template::render_body(&body, WebpageType::Index).as_str());
    html.push_str(template::FOOTER);

    let index_path = Path::new(&output_dir).join("index.html");
    std::fs::write(index_path, html)?;

    Ok(())
}
