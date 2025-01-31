use serde::Deserialize;
use std::fs;
use pulldown_cmark::{Parser, Options, html};

/// Front matter for a typical blog post (includes date).
#[derive(Debug, Deserialize)]
pub struct PostFrontMatter {
    pub title: String,
    pub date: String,
    pub author: String,
}

/// Front matter for a generic page (like About).
#[derive(Debug, Deserialize)]
pub struct PageFrontMatter {
    pub title: String,
    pub author: String,
}

/// Represents a single blog post.
#[derive(Debug)]
pub struct Post {
    /// Parsed front matter (title, date, author).
    pub front_matter: PostFrontMatter,
    /// Final HTML content after Markdown conversion.
    pub content: String,
    /// Estimated reading time (in minutes).
    pub reading_time: usize,
    /// Destination file name (e.g. "public/posts/my-title.html").
    pub file_name: String,
}

/// Represents a generic page (e.g., About page).
#[derive(Debug)]
pub struct Page {
    /// Parsed front matter (title, author).
    pub front_matter: PageFrontMatter,
    /// Final HTML content after Markdown conversion.
    pub content: String,
}

/// Parses a blog post Markdown file with front matter:
///
/// ```md
/// ---
/// title: "My Post"
/// date: "2025-01-30"
/// author: "John Doe"
/// ---
///
/// # My Post Content
/// ```
pub fn parse_post_markdown(file_path: &str) -> Result<Post, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;

    // 1. Split off the leading '---\n'
    let mut sections = content.splitn(2, "---\n");
    sections.next(); // skip the first empty part if any

    // 2. Extract front matter + remainder
    let front_matter_str = sections
        .next()
        .ok_or("Missing front matter section (--- line not found)")?;

    // 3. Split front matter from the actual Markdown body
    let mut body_sections = front_matter_str.splitn(2, "\n---\n");
    let front_matter_yaml = body_sections
        .next()
        .ok_or("Missing YAML front matter contents")?;
    let markdown_body = body_sections
        .next()
        .ok_or("Missing Markdown body after front matter")?;

    // 4. Parse front matter with Serde
    let front_matter: PostFrontMatter = serde_yaml::from_str(front_matter_yaml)?;

    // 5. Convert Markdown to HTML
    let mut html_output = String::new();
    let parser = Parser::new_ext(markdown_body, Options::all());
    html::push_html(&mut html_output, parser);

    // 6. Calculate estimated reading time (assume ~200 words/min)
    let word_count = markdown_body.split_whitespace().count();
    let reading_time = (word_count as f64 / 200.0).ceil() as usize;

    // 7. Generate a default file name in `public/posts`
    let file_name = format!(
        "public/posts/{}.html",
        front_matter.title.to_lowercase().replace(' ', "-")
    );

    Ok(Post {
        front_matter,
        content: html_output,
        reading_time,
        file_name,
    })
}

/// Parses a generic Markdown page with simpler front matter:
///
/// ```md
/// ---
/// title: "About Me"
/// author: "John Doe"
/// ---
///
/// # About Content Here
/// ```
pub fn parse_page_markdown(file_path: &str) -> Result<Page, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;

    // 1. Split off the leading '---\n'
    let mut sections = content.splitn(2, "---\n");
    sections.next(); // skip the first empty part if any

    // 2. Extract front matter + remainder
    let front_matter_str = sections
        .next()
        .ok_or("Missing front matter section (--- line not found)")?;

    // 3. Split front matter from the actual Markdown body
    let mut body_sections = front_matter_str.splitn(2, "\n---\n");
    let front_matter_yaml = body_sections
        .next()
        .ok_or("Missing YAML front matter contents")?;
    let markdown_body = body_sections
        .next()
        .ok_or("Missing Markdown body after front matter")?;

    // 4. Parse front matter with Serde
    let front_matter: PageFrontMatter = serde_yaml::from_str(front_matter_yaml)?;

    // 5. Convert Markdown to HTML
    let mut html_output = String::new();
    let parser = Parser::new_ext(markdown_body, Options::all());
    html::push_html(&mut html_output, parser);

    Ok(Page {
        front_matter,
        content: html_output,
    })
}
