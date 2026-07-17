use trpl::Html;

fn main() {
  // async is not allowed on main function so we use block on
  trpl::block_on(async {
    let title = page_title("https://pulu.me").await;
    println!("{title:?}");
  });
}

// making a function as async is the same as returning a future
async fn page_title(url: &str) -> Option<String> {
  let response = trpl::get(url).await;
  let body = response.text().await;
  Html::parse(&body)
    .select_first("title")
    .map(|title| title.inner_html())
}
