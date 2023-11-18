mod github;
mod client;
mod error;

#[cfg(test)]
mod tests {
    use super::github::client;

    #[test]
    fn test() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            println!("== test started ==");
            // client::github().await.unwrap();
            println!("== test finished ==");
        });
    }
}
