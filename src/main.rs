extern crate mdbook;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use lazy_static::lazy_static;
use regex::Regex;

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
    lazy_static! {
        // a word must have at least one letter in any language
        static ref RE: Regex = Regex::new(r"\pL").unwrap();
    }
    ch.content
        .split_whitespace()
        .filter(|&x| RE.is_match(x))
        .count()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words_title_punctuation() {
        let ch = Chapter {
            name: "".to_string(),
            content: "# some title ?".to_string(),
            number: None,
            sub_items: vec![],
            path: None,
            source_path: None,
            parent_names: vec![],
        };

        assert_eq!(count_words(&ch), 2);
    }

    #[test]
    fn test_count_words_maj() {
        let ch = Chapter {
            name: "".to_string(),
            content: "HELLO WORLD !".to_string(),
            number: None,
            sub_items: vec![],
            path: None,
            source_path: None,
            parent_names: vec![],
        };

        assert_eq!(count_words(&ch), 2);
    }

    #[test]
    fn test_count_words_compound() {
        let ch = Chapter {
            name: "".to_string(),
            content: " some-compound-word".to_string(),
            number: None,
            sub_items: vec![],
            path: None,
            source_path: None,
            parent_names: vec![],
        };

        assert_eq!(count_words(&ch), 1);
    }

    #[test]
    fn test_count_words_chinese() {
        let ch = Chapter {
            name: "".to_string(),
            content: "中國人".to_string(),
            number: None,
            sub_items: vec![],
            path: None,
            source_path: None,
            parent_names: vec![],
        };

        assert_eq!(count_words(&ch), 1);
    }

    #[test]
    fn test_count_words_not_word() {
        let ch = Chapter {
            name: "".to_string(),
            content: "3 🤩 + 🂠 ".to_string(),
            number: None,
            sub_items: vec![],
            path: None,
            source_path: None,
            parent_names: vec![],
        };

        assert_eq!(count_words(&ch), 0);
    }
}
