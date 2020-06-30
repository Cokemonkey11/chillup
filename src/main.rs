use anyhow::Result;
use reqwest::get;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use thiserror::Error;

#[derive(Debug, Error)]
enum ChillupError {
    #[error("\n{context}\n^^^\nCouldn't find an element {el:#?} in the node described above")]
    DomSchemaMismatchError { el: String, context: String },
}

impl ChillupError {
    fn of_none<T: Into<String>, U: std::fmt::Debug>(el: T, context: U) -> ChillupError {
        ChillupError::DomSchemaMismatchError {
            el: el.into(),
            context: format!("{:#?}", context),
        }
    }
}

fn dump_dependencies() -> Result<()> {
    Document::from(&get("https://github.com/topics/wurst")?.text()?[..])
        .find(Name("article"))
        .into_iter()
        .try_for_each(|repo| -> Result<()> {
            if repo
                .find(Class("topic-tag"))
                .into_iter()
                .find(|node| node.text().trim() == "dependency")
                .is_some()
            {
                let anchor = repo
                    .find(Class("f3").descendant(Name("a")))
                    .into_iter()
                    .nth(1)
                    .ok_or(ChillupError::of_none("a", repo))?;

                // Fetch description.
                let px_3 = repo
                    .find(Class("px-3"))
                    .into_iter()
                    .nth(2)
                    .ok_or(ChillupError::of_none("px-3", repo))?;
                let description = px_3
                    .find(Name("div"))
                    .into_iter()
                    .nth(0)
                    .ok_or(ChillupError::of_none("div", px_3));

                println!(
                    "https://github.com{:<50}{}",
                    anchor
                        .attr("href")
                        .ok_or(ChillupError::of_none("href", anchor))?,
                    description.map(|n| n.text().trim().to_owned()).unwrap_or_else(|_| "(No description)".into())
                );
            }

            Ok(())
        })?;

    Ok(())
}

#[paw::main]
fn main(args: paw::Args) -> Result<()> {
    let len = args.len();

    args.chain(if len == 1 {
        vec!["--help".into()].into_iter()
    } else {
        vec![].into_iter()
    })
    .skip(1)
    .filter(|arg| &arg[0..2] == "--")
    .try_for_each(|arg| match &arg[..] {
        "--help" => {
            println!("--help\t::\tThis message");

            println!("--dump\t::\tDump all dependencies found");

            println!("--index\t::\tHow to contribute to the package index");

            Ok(())
        }
        "--index" => {
            println!(
                r"
To make your own wurst library searchable, it must be:

-   Uploaded to GitHub in a public repository
-   Tagged with the `wurst` and `dependency` topics"
            );

            Ok(())
        }
        "--dump" => dump_dependencies(),
        _ => {
            println!("Couldn't understand {} - try --help?", arg);
            Ok(())
        }
    })?;

    Ok(())
}
