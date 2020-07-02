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

fn dump_dependencies(page: usize) -> Result<()> {
    let doc = Document::from(
        &get(&format!(
            "https://github.com/search?p={}&q=topic%3Awurst+topic%3Adependency",
            page
        ))?
        .text()?[..],
    );

    doc.find(Class("repo-list-item").child(Class("mt-n1")))
        .try_for_each(|repo| -> Result<()> {
            let anchor = repo
                .find(Class("f4").child(Name("a")))
                .next()
                .ok_or_else(|| ChillupError::of_none("f4 > a", repo))?;

            let description = repo
                .find(Class("mb-1"))
                .next()
                .ok_or_else(|| ChillupError::of_none("mb-1", repo));

            println!(
                "https://github.com{:<50}{}",
                anchor
                    .attr("href")
                    .ok_or_else(|| ChillupError::of_none("href", anchor))?,
                description
                    .map(|n| n.text().trim().to_owned())
                    .unwrap_or_else(|_| "(No description)".into())
            );

            Ok(())
        })?;

    if !doc
        .find(Class("next_page"))
        .next()
        .ok_or_else(|| ChillupError::of_none("next_page", doc.clone()))?
        .attr("class")
        .unwrap()
        .contains("disabled")
    {
        return dump_dependencies(page + 1);
    }

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
        "--dump" => dump_dependencies(1),
        _ => {
            println!("Couldn't understand {} - try --help?", arg);
            Ok(())
        }
    })?;

    Ok(())
}
