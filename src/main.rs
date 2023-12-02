use std::{env, thread::sleep, time::Duration};

use anyhow::{anyhow, Context, Result};
use dotenv_codegen::dotenv;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};

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
        "https://www.instagram.com/accounts/login/?next=/p/{id}"
    ))?
    .wait_until_navigated()?;
    if let Ok(el) = tab.find_element("button[tabindex='0']") {
        println!("accepting cookies");
        el.click()?;
        tab.wait_until_navigated()?;
        sleep(Duration::from_secs(1));
    }
    if tab.find_element("input[name=username]").is_ok() {
        logging_in(&tab, user, pass)?;
    }
    println!("logged in");
    std::thread::park();
    let scroll = tab.find_element_by_xpath("/html/body/div[2]/div/div/div[2]/div/div/div[1]/div[1]/div[2]/section/main/div/div[1]/div/div[2]/div/div[2]").context("scroll bar")?;
    let scroll_classes = scroll.get_attribute_value("class")?.unwrap();
    println!("scroll classes: {}", scroll_classes);
    let loadingwheel = "div[data-visualcompletion=loading-state]";
    let jsscroll = "let el = document.querySelector('');
        el.scrollTop = el.scrollHeight;
    ";
    loop {
        tab.evaluate(jsscroll, true)?;
        sleep(Duration::from_secs(1));
    }
    let comments_text =
        tab.find_elements_by_xpath("//div/div/div[2]/div[1]/div[1]/div/div[2]/span")?;
    println!("founds {} comments", comments_text.len());
    for comments in comments_text {
        let text = comments.get_inner_text()?;
        println!("comment: {}", text);
    }
    std::thread::park();
    Ok(())
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
