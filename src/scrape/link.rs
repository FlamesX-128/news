use crabquery::Document;
use std::error::Error;
use terminal_spinners::{SpinnerBuilder, DOTS, DOTS2, MOON};

#[derive(Debug)]
pub struct NewsLink {
    pub title: String,
    pub link: String,
}
/// Search for javascript news of a specific issue
/// And return two arrays one of the news object and other of options of news to search
pub async fn get_js_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&MOON)
        .text("Fetching JavaScript news")
        .start();

    let response = reqwest::get(url).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let elements_li = doc.select(".mainlink");
    let mut vec_issues: Vec<NewsLink> = vec![];
    for elem in elements_li {
        let uri = elem
            .select("a")
            .first()
            .expect("fail to get first element of uri")
            .attr("href")
            .expect("fail to get attr href");

        let mut title = elem
            .children()
            .first()
            .expect("fail to get first element")
            .text()
            .expect("fail to conver a text");

        if title.len() < 6 {
            title = elem
                .children()
                .get(1)
                .expect("fail to get first element children title")
                .text()
                .expect("fail to convert a text title children");
        }

        let new = NewsLink { title, link: uri };
        vec_issues.push(new)
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_issues, issues_options))
}

/// Search for rust news of a specific issue
/// And return two arrays one of the news object and other of options of news to search
pub async fn get_rs_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text(" Fetching Rust news")
        .start();
    let response = reqwest::get(url).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let post = doc.select(".post-content");
    let elem = post.first().unwrap();
    let elements_li = elem.select("li");
    let mut vec_issues: Vec<NewsLink> = vec![];
    for elem in elements_li {
        let uri = elem
            .children()
            .first()
            .expect("Failed to get first element")
            .attr("href");
        let title = elem
            .children()
            .first()
            .expect("Failed to get first element")
            .text()
            .unwrap();

        if let Some(link) = uri {
            if !title.is_empty() {
                let new = NewsLink { title, link };
                vec_issues.push(new)
            }
        }
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_issues, issues_options))
}

pub async fn get_go_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&DOTS2)
        .text(" Fetching Go news")
        .start();

    let response = reqwest::get(url).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let elements_li = doc.select(".mainlink");
    let mut vec_issues: Vec<NewsLink> = vec![];
    for elem in elements_li {
        let uri = elem
            .select("a")
            .first()
            .expect("fail to get first element of uri")
            .attr("href")
            .expect("fail to get attr href");

        let mut title = elem
            .children()
            .first()
            .expect("fail to get first element")
            .text()
            .expect("fail to conver a text");

        if title.len() < 6 {
            title = elem
                .children()
                .get(1)
                .expect("fail to get first element children title")
                .text()
                .expect("fail to convert a text title children");
        }

        let new = NewsLink { title, link: uri };
        vec_issues.push(new)
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_issues, issues_options))
}

pub async fn get_py_news(url: &str) -> Result<(Vec<NewsLink>, Vec<String>), Box<dyn Error>> {
    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text(" Fetching Python news")
        .start();
    let response = reqwest::get(url).await?;

    let text = response.text().await?;

    let doc = Document::from(text);

    let elements_li = doc.select("span");
    let mut vec_issues: Vec<NewsLink> = vec![];
    let mut links: Vec<String> = vec![];
    for elem in elements_li {
        if let Some(value) = elem.children().first() {
            let source = value.attr("href");
            if let Some(link) = source {
                if links.iter().any(|l| l.contains(&link)) == false {
                    let title = elem
                        .children()
                        .first()
                        .expect("Failed to get first child element")
                        .text()
                        .expect("Failed to tranform to String");
                    let new = NewsLink {
                        title,
                        link: link.clone(),
                    };

                    vec_issues.push(new)
                }
                links.push(link);
            }
        }
    }

    let issues_options = vec_issues.iter().map(|new| new.title.clone()).collect();

    handle.done();

    Ok((vec_issues, issues_options))
}
