#[macro_use] extern crate failure;

use failure::Error;
use reqwest::get;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};


#[derive(Debug, Fail)]
enum ChillupError {
    #[fail(display = "Couldn't find an element {}", el)]
    NoneError {
        el: String,
    }
}

fn dump_dependencies() -> Result<(), Error> {
    let _: () = Document::from(
        &get("https://github.com/topics/wurst")?.text()?[..]
    ).find(Name("article"))
    .into_iter()
    .try_for_each(|repo| -> Result<(), Error> {
        let container = repo
            .find(Class("topics-row-container"))
            .next()
            .ok_or(ChillupError::NoneError{ el: "topics-row-container".into() })?;

        if container.find(Class("topic-tag"))
            .into_iter()
            .find(|node| node.text().trim() == "dependency")
            .is_some()
        {
            let anchor      = repo.find(Class("f3").descendant(Name("a")))
                .into_iter()
                .next()
                .ok_or(ChillupError::NoneError{ el: "a".into() })?;
            let description = repo.find(Class("text-gray"))
                .into_iter()
                .next()
                .ok_or(ChillupError::NoneError{ el: "text-gray".into() })?;
            
            println!("https://github.com{}\t{}", anchor.attr("href").ok_or(ChillupError::NoneError{ el: "href".into() })?, description.text().trim());
        }

        Ok(())
    })?;

    Ok(())
}

#[paw::main]
fn main(args: paw::Args) -> Result<(), Error> {
    let len = args.len();

    args.chain(
        if len == 1
        { vec!["--help".into()].into_iter() }
        else { vec![].into_iter() }
    )
        .skip(1)
        .filter(|arg| &arg[0..2] == "--")
        .try_for_each(
            |arg| {
                match &arg[..] {
                    "--help" => {
                        println!("--help\t::\tThis message");

                        println!("--dump\t::\tDump all dependencies found");

                        println!("--index\t::\tHow to contribute to the package index");
                        
                        Ok(())
                    },
                    "--index" => {
                        println!(r"
To make your own wurst library searchable, it must be:

-   Uploaded to GitHub in a public repository
-   Tagged with the `wurst` and `dependency` topics"
                        );

                        Ok(())
                    }
                    "--dump" => dump_dependencies(),
                    _ => { println!("Couldn't understand {} - try --help?", arg); Ok(()) },
                }
            }
        )
    ?;

    Ok(())
}
