//! # Usage
//!
//! This is just about the smallest possible way to do inference. To fetch a model from hugging face:
//!
//! ```console
//! git clone --recursive https://github.com/eugenehp/bitnet-cpp-rs
//! cd bitnet-cpp-rs/examples/usage
//! wget https://huggingface.co/eugenehp/Llama3-8B-1.58-100B-tokens-GGUF/blob/main/ggml-model-i2_s.gguf
//! cargo run --example usage -- ggml-model-i2_s.gguf
//! ```
use bitnet_cpp::context::params::LlamaContextParams;
use bitnet_cpp::llama_backend::LlamaBackend;
use bitnet_cpp::llama_batch::LlamaBatch;
use bitnet_cpp::model::params::LlamaModelParams;
use bitnet_cpp::model::LlamaModel;
use bitnet_cpp::model::{AddBos, Special};
use bitnet_cpp::token::data_array::LlamaTokenDataArray;
use bitnet_cpp::token::LlamaToken;
use bitnet_cpp_sys::{
    llama_sampler_apply, llama_sampler_chain_add, llama_sampler_chain_default_params,
    llama_sampler_chain_init, llama_sampler_init_greedy, llama_sampler_sample,
};
use std::io::Write;

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
fn main() {
    let model_path = std::env::args().nth(1).expect("Please specify model path");
    let backend = LlamaBackend::init().unwrap();
    let params = LlamaModelParams::default();

    let prompt =
        "<|im_start|>user\nHello! how are you?<|im_end|>\n<|im_start|>assistant\n".to_string();
    LlamaContextParams::default();
    let model =
        LlamaModel::load_from_file(&backend, model_path, &params).expect("unable to load model");
    let ctx_params = LlamaContextParams::default();
    let mut ctx = model
        .new_context(&backend, ctx_params)
        .expect("unable to create the llama_context");
    let tokens_list = model
        .str_to_token(&prompt, AddBos::Always)
        .unwrap_or_else(|_| panic!("failed to tokenize {prompt}"));
    let n_len = 64;

    // create a llama_batch with size 512
    // we use this object to submit token data for decoding
    let mut batch = LlamaBatch::new(512, 1);

    let last_index = tokens_list.len() as i32 - 1;
    for (i, token) in (0_i32..).zip(tokens_list.into_iter()) {
        // llama_decode will output logits only for the last token of the prompt
        let is_last = i == last_index;
        batch.add(token, i, &[0], is_last).unwrap();
    }
    ctx.decode(&mut batch).expect("llama_decode() failed");

    let mut n_cur = batch.n_tokens();

    // The `Decoder`
    let mut decoder = encoding_rs::UTF_8.new_decoder();
    let params = unsafe { llama_sampler_chain_default_params() };
    let mut smpl = unsafe { llama_sampler_chain_init(params) };

    // https://github.com/ggerganov/llama.cpp/blob/master/examples/simple/simple.cpp#L124
    unsafe {
        llama_sampler_chain_add(smpl, llama_sampler_init_greedy());
    };

    while n_cur <= n_len {
        // sample the next token
        {
            let candidates = ctx.candidates_ith(batch.n_tokens() - 1);

            let mut candidates_p = LlamaTokenDataArray::from_iter(candidates, false);

            // sample the most likely token
            // let new_token_id = ctx.sample_token_greedy(candidates_p);
            // new_token_id = llama_sampler_sample(smpl, ctx, -1);
            let mut data_arr = bitnet_cpp_sys::llama_token_data_array {
                data: candidates_p
                    .data
                    .as_mut_ptr()
                    .cast::<bitnet_cpp_sys::llama_token_data>(),
                size: candidates_p.data.len(),
                selected: -1,
                sorted: candidates_p.sorted,
            };

            unsafe {
                llama_sampler_apply(smpl, &mut data_arr);
            };

            let new_token = unsafe { llama_sampler_sample(smpl, ctx.context.as_ptr(), -1) };
            let new_token_id = LlamaToken::new(new_token);

            // is it an end of stream?
            if new_token_id == model.token_eos() {
                eprintln!();
                break;
            }

            let output_bytes = model
                .token_to_bytes(new_token_id, Special::Tokenize)
                .unwrap();
            // use `Decoder.decode_to_string()` to avoid the intermediate buffer
            let mut output_string = String::with_capacity(32);
            let _decode_result = decoder.decode_to_string(&output_bytes, &mut output_string, false);
            print!("{output_string}");
            std::io::stdout().flush().unwrap();

            batch.clear();
            batch.add(new_token_id, n_cur, &[0], true).unwrap();
        }

        n_cur += 1;

        ctx.decode(&mut batch).expect("failed to eval");
    }
}
