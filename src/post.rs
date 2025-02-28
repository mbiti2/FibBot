use std::fmt::format;

use reqwest::{header, Client, Error, Response, Url};
use serde::Serialize;
#[derive(Serialize)]
struct Comment {
    body: String,
}

#[tokio::main]
pub async fn post_comment(fib_num: u128, owner: &str, repo: &str, pr_num: u32, token: String ) -> Result<String, Error> { 
    // https://api.github.com/repos/FibBot/issues/1/comments

    let client = Client::new();


    let comment = format!("The fibonnaci number is : {}", fib_num);
    let comment = Comment { body: comment.to_string() };
    
    let response = client
    .post(format!("https://api.github.com/repos/{repo}/issues/{pr_num}/comments" ))
  .header("Accept", "application/vnd.github+json")
   .header("Authorization", format!("{}", token).as_str()) 
   .header("X-GitHub-Api-Version", "2022-11-28") 
   .header("User-Agent", format!("{}", owner))
    .json(&comment)
    .send()
    .await?;

    println!("{:#?}", response);

   Ok(comment.body)

}
