extern crate mdbook;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use mdbook::book::{BookItem, Chapter};
use mdbook::renderer::RenderContext;
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct WordcountConfig {
    pub ignores: Vec<String>,
}

fn count_words(ch: &Chapter) -> usize {
    ch.content.split_whitespace().count()
}

fn get_config(ctx: &RenderContext) -> WordcountConfig {
    return ctx
        .config
        .get_deserialized_opt("output.wordcount")
        .unwrap()
        .unwrap();
}

fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    let _ = fs::create_dir_all(&ctx.destination);
    let mut f = File::create(ctx.destination.join("wordcounts.txt")).unwrap();
    let cfg = get_config(&ctx);

    let mut total_words = 0;

    for item in ctx.book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            if cfg.ignores.contains(&ch.name) {
                continue;
            }

            let words = count_words(ch);
            total_words += words;
            println!("{}: {}", ch.name, words);
            writeln!(f, "{}: {}", ch.name, words).unwrap();
        }
    }
    println!("---------");
    println!("Total: {}", total_words);
    writeln!(f, "Total : {}", total_words).unwrap();
}
