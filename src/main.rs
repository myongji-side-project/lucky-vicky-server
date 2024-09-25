use async_openai::types::CreateCompletionRequestArgs;
use dotenv::dotenv;

// TODO: OPENAI_API_KEY (https://platform.openai.com/account/api-keys) 환경변수 설정
#[tokio::main]
async fn main() {
    dotenv().expect("Fail to get .env");
    let client = async_openai::Client::new();
    let request = CreateCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .prompt("슈게이징 장르를 설명해줘")
        .max_tokens(40_u32)
        .build()
        .expect("Fail to request to gpt");
    let response = client
        .completions()
        .create(request)
        .await
        .expect("Fail to complete result");
    for choice in response.choices {
        println!("{}", choice.text);
    }
}
