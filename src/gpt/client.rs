
use anyhow::Error;
use futures_util::Stream;
use reqwest::{header::{HeaderMap, AUTHORIZATION, HeaderValue}, Response};

use crate::gpt::{types::{InboundResponseChunk, InboundChunkPayload}, err};

use super::types::{ServerResponse, CompletionRequest, Completion, Message, Role, ResponseChunk, Model, Prompt};

pub type Result<T> = std::result::Result<T, Error>;

pub struct Client {
    client: reqwest::Client,
    url: String,
    model: Model,
    prompt: String
}

impl Client {
    pub fn new(model: Model) -> crate::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(format!("Bearer {}", model.api_key).as_bytes()).expect("Problem converting header"),
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;
        Ok(Self {
            client, 
            url: "https://api.openai.com/v1/chat/completions".into(), 
            model,
            prompt: "".into()
        })
    }
    pub fn add_prompt(&mut self, prompt: Prompt) {
        self.prompt = prompt.content;
    }
    pub async fn post_streaming(
        &self, 
        question: String
    ) -> Result<impl Stream<Item = ResponseChunk>> {

        let resp = self.client
            .post(self.url.clone())
            .json(&CompletionRequest {
                model: self.model.model.as_str(),
                messages: &vec![ Message { 
                    role: Role::User,
                    content: format!("Prompt: {}, Question: {}", self.prompt, question)
                }],
                stream: true
            })
            .send()
            .await?;

        Self::process_streaming_response(resp)
    }

    fn process_streaming_response(
        response: Response,
    ) -> crate::Result<impl Stream<Item = ResponseChunk>> {
        use eventsource_stream::Eventsource;
        use futures_util::StreamExt;
        // also handles errors
        response
            .error_for_status()
            .map(|response| {
                let response_stream = response.bytes_stream().eventsource();
                response_stream.map(move |part| {
                    let chunk = &part.expect("Stream closed abruptly!").data;
                    if chunk == "[DONE]" {
                        return ResponseChunk::Done;
                    }
                    let data: InboundResponseChunk = serde_json::from_str(chunk)
                        .expect("Invalid inbound streaming response payload!");
                    let choice = data.choices[0].to_owned();
                    match choice.delta {
                        InboundChunkPayload::AnnounceRoles { role } => {
                            ResponseChunk::BeginResponse {
                                role,
                                response_index: choice.index,
                            }
                        }
                        InboundChunkPayload::StreamContent { content } => ResponseChunk::Content {
                            delta: content,
                            response_index: choice.index,
                        },
                        InboundChunkPayload::Close {} => ResponseChunk::CloseResponse {
                            response_index: choice.index,
                        },
                    }
                })
            })
            .map_err(Error::from)
    }
}