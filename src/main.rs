use std::{
    cell::Ref, env, fs::File, io::Write, os::unix::fs::FileExt, thread::sleep, time::Duration,
};

use anyhow::{anyhow, Context, Result};
use dotenv_codegen::dotenv;
use headless_chrome::{Browser, Element, LaunchOptionsBuilder, Tab};

fn main() -> Result<()> {
    let user = dotenv!("insta_user");
    let pass = dotenv!("insta_pass");
    let url = env::args().nth(1).context("didn't give argument!")?;
    let start_requirement = "https://www.instagram.com/p/";
    if !url.starts_with(start_requirement) {
        return Err(anyhow!(
            "Please give a instagram post url. it must start with {start_requirement}"
        ));
    }
    let id = &url[start_requirement.len()..].trim_matches('/');
    println!("scraping {id}");
    let headless = env::var("HEADLESS").is_err();
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(headless)
            .user_data_dir(Some("/tmp/insta-scraper-chrome-data".into()))
            .build()?,
    )?;
    let tab = browser.new_tab()?;
    tab.set_user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 17_1_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Mobile/15E148 Safari/604.1", None, None)?;
    tab.navigate_to(&format!(
        "https://www.instagram.com/accounts/login/?next=/p/{id}/comments"
    ))?
    .wait_until_navigated()?;
    if let Ok(el) = tab.find_element_by_xpath(
        "/html/body/div[6]/div[1]/div/div[2]/div/div/div/div/div[2]/div/div[3]/div[1]/div/button",
    ) {
        println!("accepting cookies");
        el.click()?;
        tab.wait_until_navigated()?;
        sleep(Duration::from_secs(2));
    }
    if tab.find_element("input[name=username]").is_ok() {
        logging_in(&tab, user, pass)?;
    }
    println!("logged in");
    let scroll_xpath = "/html/body/div[2]/div/div/div[2]/div/div/div[1]/div[1]/div[2]/section/main/div/div/div/div[1]";
    let jsscroll = format!("el = document.evaluate('{scroll_xpath}', document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null).singleNodeValue; el.scrollTop = el.scrollHeight;");
    let loadingwheel = "div[data-visualcompletion=loading-state]";
    loop {
        let mut quit = true;
        _ = tab.find_element(loadingwheel).map(|_| quit = false);
        tab.evaluate(&jsscroll, true)?;
        _ = tab.find_element(loadingwheel).map(|_| quit = false);
        sleep(Duration::from_millis(100));
        _ = tab.find_element(loadingwheel).map(|_| quit = false);
        if quit {
            break;
        }
    }
    let spans =
        tab.find_elements_by_xpath("//div/div/div/div/div[2]/div/div[1]/div/div/span[2]/span[1]")?;
    println!("founds {} comments", spans.len());
    let comments = spans
        .iter()
        .map(Element::get_inner_text)
        .collect::<Result<Vec<_>>>()
        .unwrap();
    write_comments_csv(id, comments.iter().map(AsRef::as_ref));
    print_stats(&comments);
    Ok(())
}

fn print_stats(comments: &[String]) {
    todo!()
}

fn write_comments_csv<'a, I: Iterator<Item = &'a str>>(id: &str, comments: I) {
    let filename = format!("comments-{id}.csv");
    let mut file = match File::create(filename.clone()) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("ERROR, open file {filename}: {e}");
            return;
        }
    };
    for comment in comments {
        file.write_all(comment.as_bytes()).unwrap();
        file.write_all(&[b',']).unwrap();
    }
    println!("wrote comments to {filename}");
}

fn logging_in(tab: &Tab, user: &str, pass: &str) -> Result<()> {
    println!("logging in");
    tab.find_element("input[name=username]")?.type_into(user)?;
    tab.find_element("input[name=password]")?.type_into(pass)?;
    tab.find_element("button[type=submit]")?.click()?;
    while tab
        .get_url()
        .starts_with("https://www.instagram.com/accounts/login")
    {
        sleep(Duration::from_millis(100));
    }
    tab.wait_until_navigated()?;
    tab.find_element("button[type=button]")?.click()?;
    tab.wait_until_navigated()?;
    Ok(())
}
