use kalosm::language::*;
#[tokio::main]
async fn main() {
    let llm = Llama::new().await.expect("Fail to get llama model");
    let prompt = "Write 300 word essay of shoegaze which is music genre.";

    let mut stream = llm
        .stream_text(prompt)
        .with_max_length(1000)
        .await
        .expect("Fail to get result");
    stream
        .to_std_out()
        .await
        .expect("Fail to write result on stdout");
}
