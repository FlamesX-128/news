//! # News
//!
//! `News` is a cli program to watch developer news from the terminal

use crabquery::Document;
use inquire::Select;
use page::markdown::{generate_view, get_markdonwwn_content};
use std::{error::Error, str::FromStr};

pub mod page;

#[derive(Debug)]
struct NewsLink {
    title: String,
    link: String,
}

#[derive(Debug)]
struct Issue {
    title: String,
    link: String,
}

#[derive(Clone, Debug)]
enum View {
    Web,
    Terminal,
}

impl FromStr for View {
    type Err = ();

    fn from_str(input: &str) -> Result<View, Self::Err> {
        match input {
            "Web" => Ok(View::Web),
            "Terminal" => Ok(View::Terminal),
            _ => Err(()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = vec!["Check Javascript News", "Exit"];

    let ans = Select::new("What's do you like to do?", options).prompt();

    match ans {
        Ok(choice) => match choice {
            "Exit" => std::process::exit(0),
            _ => println!("Js News"),
        },
        Err(_) => {
            println!("There was an error, please try again");
            std::process::exit(0);
        }
    }
    let (issues, issues_options) = get_js_issues_news().await?;

    let ans = Select::new("What new do you like to watch?", issues_options)
        .prompt()
        .unwrap();

    let new_item = issues.iter().find(|new| new.title == ans);

    if let Some(value) = new_item {
        let (news, options) = get_js_news(value.link.as_str()).await?;
        let answer = Select::new("What new do you like to watch?", options)
            .prompt()
            .unwrap();

        let new_struct = news.iter().find(|new| new.title == answer);

        if let Some(new) = new_struct {
            let link = &new.link;
            let response = reqwest::get(link).await?;
            let url = response.url();

            // if is a video of youtube
            if url.domain().unwrap().contains("youtube") {
                if webbrowser::open(new.link.as_str()).is_ok() {}
                return Ok(());
            }

            let view_select = Select::new("What view do you like to do?", vec!["Web", "Terminal"])
                .with_help_message("Enter the view of the new")
                .prompt()
                .unwrap();

            let view = View::from_str(view_select).unwrap();
            match view {
                View::Terminal => {
                    println!("{link}");

                    let html = response.text().await?;
                    let markdown = get_markdonwwn_content(&html);
                    generate_view(markdown.as_str()).unwrap();
                }
                View::Web => if webbrowser::open(new.link.as_str()).is_ok() {},
            }
        }
    }

    println!("res: {ans}");

    Ok(())
}

/// Search for javascript weekly issues news
/// Return a array of Issues and options of that issues to search them
async fn get_js_issues_news() -> Result<(Vec<Issue>, Vec<String>), Box<dyn Error>> {
    const JAVASCRIPT_WEEKLY_URL: &str = "https://javascriptweekly.com/issues";

    let response = reqwest::get(JAVASCRIPT_WEEKLY_URL).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let issues = doc.select(".issue");

    let mut vec_issues: Vec<Issue> = vec![];
    for issue in issues {
        let url = issue.children().first().unwrap().attr("href").unwrap();
        let number_issue = url.split('/').last().unwrap();
        let url_completed = format!("{JAVASCRIPT_WEEKLY_URL}/{number_issue}");
        let name = issue.text().unwrap();
        let new = Issue {
            title: name,
            link: url_completed,
        };
        vec_issues.push(new);
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    Ok((vec_issues, issues_options))
}

/// Search for javascript news of a specific issue
/// And return two arrays one of the news object and other of options of news to search
async fn get_js_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let response = reqwest::get(url).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let elements_li = doc.select(".desc");
    let mut vec_issues: Vec<NewsLink> = vec![];
    for elem in elements_li {
        let uri = elem
            .select("a")
            .first()
            .expect("algo")
            .attr("href")
            .unwrap();

        let mut title = elem
            .children()
            .first()
            .unwrap()
            .children()
            .first()
            .unwrap()
            .text()
            .unwrap();

        if title.len() < 10 {
            title = elem
                .children()
                .first()
                .unwrap()
                .children()
                .get(1)
                .unwrap()
                .text()
                .unwrap();
        }

        let new = NewsLink { title, link: uri };
        vec_issues.push(new)
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    Ok((vec_issues, issues_options))
}
