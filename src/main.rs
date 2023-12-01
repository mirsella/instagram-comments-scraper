use std::{thread::sleep, time::Duration};

use anyhow::{anyhow, Context, Result};
use dotenv_codegen::dotenv;
use headless_chrome::{Browser, LaunchOptionsBuilder};

fn main() -> Result<()> {
    let user = dotenv!("insta_user");
    let pass = dotenv!("insta_pass");
    let url = std::env::args().nth(1).context("didn't give argument!")?;
    let start_requirement = "https://www.instagram.com/p/";
    if !url.starts_with(start_requirement) {
        return Err(anyhow!(
            "Please give a instagram post url. it must start with {start_requirement}"
        ));
    }
    println!("scraping {url}");
    // let browser = Browser::default().unwrap();
    let browser = Browser::new(LaunchOptionsBuilder::default().headless(false).build()?)?;
    let tab = browser.new_tab()?;
    tab.navigate_to("https://instagram.com")?
        .wait_until_navigated()?;
    if let Ok(el) = tab.find_element("button[tabindex='0']") {
        println!("accepting cookies");
        el.click()?;
    }
    tab.wait_until_navigated()?;
    sleep(Duration::from_secs(1));
    println!("logging in");
    tab.find_element("input[name=username]")?.type_into(user)?;
    tab.find_element("input[name=password]")?.type_into(pass)?;
    tab.find_element("button[type=submit]")?.click()?;
    tab.wait_until_navigated()?;
    tab.navigate_to(&url)?.wait_until_navigated()?;
    sleep(Duration::from_secs(50));
    Ok(())
}
