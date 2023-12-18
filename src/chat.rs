use chatgpt::prelude::*;
use chatgpt::types::CompletionResponse;
use std::io::{stdout, Write};
use futures_util::stream::StreamExt;
use rand::Rng;


#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}

pub async fn ask(message: String) -> Result<Vec<ChatMessage>> {
    let key = std::env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY not found.");
    // let session = rand::thread_rng().gen_range(0..100);
    // let hist_file = format!("{id}-last-conversation.json", id=session.to_string());
    // let instruct = std::env::var("CHATGPT_INSTRUCT").expect("CHATGPT_INSTRUCT not found.");
    let hist_file = "my-conversation.json";

    let instruct = "CHATGPT_INSTRUCT=Act as a user friendly tech support consultant for an IT
    company that deploys software in docker containers, your name is Byte Boffin, you start your
    first message with greetings, presenting yourself and asking client's name only once,
    you respond with short messages";

    // Creating a new ChatGPT client.
    // Note that it requires an API key, and uses
    // tokens from your OpenAI API account balance.
    // let client = ChatGPT::new(key)?;
    let client = ChatGPT::new_with_config(
        key,
        ModelConfigurationBuilder::default()
            .engine(ChatGPTEngine::Gpt35Turbo_0301)
            .build()
            .unwrap(),
    )?;

    let mut conversation: Conversation = client.new_conversation_directed(instruct);
    let mut stream = conversation
        .send_message_streaming(message)
        .await?;

    // Iterating over a stream and collecting the results into a vector
    let mut output: Vec<ResponseChunk> = Vec::new();
    while let Some(chunk) = stream.next().await {
        match chunk {
            ResponseChunk::Content {
                delta,
                response_index,
            } => {
                // Printing part of response without the newline
                // Manually flushing the standard output, as `print` macro does not do that
                stdout().lock().flush().unwrap();
                output.push(ResponseChunk::Content {
                    delta,
                    response_index,
                });
            }
            // We don't really care about other types, other than parsing them into a ChatMessage later
            other => {
                // println!("{:?}", other);
                output.push(other)
            }
        }
    }

    // Parsing ChatMessage from the response chunks and saving it to the conversation history
    let response = ChatMessage::from_response_chunks(output);

    // Parsing ChatMessage from the response chunks and saving it to the conversation history
    conversation.history.push(response[0].to_owned());
    conversation.save_history_json(hist_file).await?;

    Ok(response)
}