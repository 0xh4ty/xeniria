use clap::{Parser, Subcommand};
use std::fs;
use std::io::Write;

// Import our custom modules
mod markdown;
use markdown::{parse_page_markdown, parse_post_markdown, Post};

// Import the server module
mod server;
use server::start_server;

#[derive(Parser)]
#[command(name = "Xeniria")]
#[command(about = "A minimal static site generator written in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the static site (parse Markdown & generate HTML)
    Build,
    /// Start a local server to preview the site at http://localhost:8080
    Serve,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => {
            println!("Building site...");

            // Ensure `public/posts` folder exists
            fs::create_dir_all("public/posts")
                .expect("Failed to create or verify public/posts directory");

            // Collect blog posts to build index.html
            let mut posts_collected: Vec<Post> = Vec::new();

            // Scan `content/` for .md files
            if let Ok(entries) = fs::read_dir("content") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("md") {
                        let file_path = path.to_string_lossy().to_string();

                        // Check if this file is `about.md`
                        if file_path.ends_with("about.md") {
                            generate_about(&file_path);
                        } else {
                            // Treat all other .md files as blog posts
                            match parse_post_markdown(&file_path) {
                                Ok(post) => {
                                    let html = format!(
                                        "<html>
                                            <head>
                                                <title>{title}</title>
                                            </head>
                                            <body>
                                                <h1>{title}</h1>
                                                <p><strong>By {author}</strong> - {date}</p>
                                                <p>Estimated Reading Time: {read_time} min</p>
                                                {content}
                                            </body>
                                        </html>",
                                        title = post.front_matter.title,
                                        author = post.front_matter.author,
                                        date = post.front_matter.date,
                                        read_time = post.reading_time,
                                        content = post.content,
                                    );

                                    let mut file = fs::File::create(&post.file_name)
                                        .expect("Failed to create post file");
                                    file.write_all(html.as_bytes())
                                        .expect("Failed to write post file");

                                    println!("Generated: {}", post.file_name);

                                    // Add to list for index.html
                                    posts_collected.push(post);
                                }
                                Err(e) => {
                                    println!("Error parsing post {}: {}", file_path, e);
                                }
                            }
                        }
                    }
                }
            }

            // Generate index.html to link to all posts
            generate_index(&posts_collected);

            println!("Site build complete!");
        }

        Commands::Serve => {
            println!("Starting server at http://localhost:8080...");
            let port = 8464;
            if let Err(e) = start_server(port) {
                eprintln!("Server error: {}", e);
            }
        }
    }
}

/// Generate `about.html` from `about.md`
fn generate_about(file_path: &str) {
    match parse_page_markdown(file_path) {
        Ok(page) => {
            let about_html = format!(
                "<html>
                    <head>
                        <title>{title}</title>
                    </head>
                    <body>
                        <h1>{title}</h1>
                        <p>By {author}</p>
                        {content}
                    </body>
                </html>",
                title = page.front_matter.title,
                author = page.front_matter.author,
                content = page.content
            );

            let mut file = fs::File::create("public/about.html")
                .expect("Failed to create about.html");
            file.write_all(about_html.as_bytes())
                .expect("Failed to write about.html");

            println!("Generated: public/about.html");
        }
        Err(e) => {
            println!("Error parsing About page {}: {}", file_path, e);
        }
    }
}

/// Generate `index.html` listing all blog posts
fn generate_index(posts: &Vec<Post>) {
    let mut index_html = String::from(
        "<html>
            <head>
                <title>Xeniria Blog</title>
            </head>
            <body>
                <h1>Xeniria Blog</h1>
                <ul>",
    );

    for post in posts {
        let link_path = post.file_name.replace("public/", "");
        index_html.push_str(&format!(
            "<li>
                <a href=\"{link}\">{title}</a>
                - {date} - {read_time} min read
            </li>",
            link = link_path,
            title = post.front_matter.title,
            date = post.front_matter.date,
            read_time = post.reading_time
        ));
    }

    index_html.push_str(
        "</ul>
        <p><a href=\"about.html\">About</a></p>
        </body>
        </html>",
    );

    let mut file =
        fs::File::create("public/index.html").expect("Failed to create public/index.html");
    file.write_all(index_html.as_bytes())
        .expect("Failed to write index.html");

    println!("Generated: public/index.html");
}
