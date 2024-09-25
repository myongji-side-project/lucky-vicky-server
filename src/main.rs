use kalosm::language::*;
// TODO: OPENAI_API_KEY (https://platform.openai.com/account/api-keys) 환경변수 설정
#[tokio::main]
async fn main() {
    let llm = Gpt3_5::default();
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
