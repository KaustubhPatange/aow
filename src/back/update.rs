use {
    serde_json::{Value},
    regex::Regex,
    back::{command::VERSION, config::Config},
};

/**
* To check version for new version once in a day.
*/
pub async fn fetch_new_version() {
    if !Config::need_to_check_update() {
        return;
    }

    let client = reqwest::Client::new();
    let res = client.get("https://api.github.com/repos/KaustubhPatange/aow/releases")
        .header("User-Agent", "aow")
        .send()
        .await;
    match res {
        Ok(value) => {
            let body: String = value.text().await.unwrap();
            let re: Regex = Regex::new(r"[.v]").unwrap();
            let u: Value = serde_json::from_str(body.as_str()).unwrap();
            let version = u[0]["tag_name"].as_str().unwrap();

            let new: i8 = re.replace_all(version, "").parse::<i8>().unwrap();
            let old: i8 = re.replace_all(VERSION, "").parse::<i8>().unwrap();

            if new > old {
                println!("Hint: New version {} is available to download from GitHub repository.", version);
            }
        }
        Err(_) => {}
    }
}