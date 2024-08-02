use anyhow::Result;
use clap::Parser;
use comfy_table::Table;
use futures_util::TryStreamExt;
use tokio::pin;

fn optional_to_string_or_empty<T: ToString>(t: Option<T>) -> String {
    t.map(|v| v.to_string()).unwrap_or_else(|| "".into())
}

fn insert_newlines(s: &str, interval: usize) -> String {
    s.chars() // Get an iterator of (index, char)
        .fold((String::new(), 0), |(mut acc, mut count), ch| {
            if count >= interval && ch.is_whitespace() {
                acc.push('\n'); // Insert newline at whitespace after every 'interval' characters
                count = 0; // Reset count after inserting newline
            } else {
                acc.push(ch); // Push character to accumulator
                count += ch.len_utf8(); // Increment count by character length
            }
            (acc, count)
        })
        .0 // Get the accumulated string
}

async fn recursive_dump_dependencies(
    use_table: bool,
    include_archived: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let octocrab = octocrab::instance();
    let search_results = octocrab
        .search()
        .repositories("topic:wurst topic:dependency")
        .sort("stars")
        .order("desc")
        .send()
        .await?;
    let search_result_stream = search_results.into_stream(&octocrab);

    pin!(search_result_stream);

    let mut table = Table::new();
    table.set_header(vec!["Stars", "URL", "Description"]);

    if !use_table {
        println!("Stars\t{:60}\tDescription", "URL");
    }

    while let Some(repo) = search_result_stream.try_next().await? {
        if repo.archived.unwrap_or(false) && !include_archived {
            continue;
        }

        let stars = repo.stargazers_count.unwrap_or(0);
        let url = optional_to_string_or_empty(repo.html_url);
        let description = insert_newlines(&optional_to_string_or_empty(repo.description), 60);

        if use_table {
            table.add_row(vec![stars.to_string(), url, description]);
        } else {
            println!(
                "{}\t{:60}\t{}",
                stars,
                url,
                &description
                    .chars()
                    .enumerate()
                    .map(|(idx, chr)| if idx == 59 { '…' } else { chr })
                    .take(60)
                    .collect::<String>()
            );
        }
    }

    if use_table {
        table.load_preset(comfy_table::presets::UTF8_FULL_CONDENSED);
        table.remove_style(comfy_table::TableComponent::HorizontalLines);
        table.remove_style(comfy_table::TableComponent::MiddleIntersections);

        println!("{table}");
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Dump all dependencies found
    #[arg(long)]
    dump: bool,

    /// How to contribute to the package index
    #[arg(long)]
    index: bool,

    /// When enabled, dumped pages are in table format
    #[arg(long)]
    table: bool,

    /// When enabled, archived repositories are included
    #[arg(long)]
    archive: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match (args.dump, args.index) {
        (true, _) => {
            recursive_dump_dependencies(args.table, args.archive).await?;

            Ok::<(), Box<dyn std::error::Error>>(())
        }
        (_, true) => {
            println!(
                r"
To make your own wurst library searchable, it must be:

-   Uploaded to GitHub in a public repository
-   Tagged with the `wurst` and `dependency` topics"
            );

            Ok(())
        }
        _ => {
            println!("?? wyd\nTry --help");

            Ok(())
        }
    }?;

    Ok(())
}
