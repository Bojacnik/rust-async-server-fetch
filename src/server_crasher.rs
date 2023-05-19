use std::task::Poll;

use futures::FutureExt;

async fn contact(url: String) {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to fetch the response");

    let response_text = response.text().await;
    match response_text {
        Ok(value) => println!("{}", value),
        Err(_) => panic!("Failed to fetch data"),
    }
}

pub fn start(threads: u32, url: String) {
    let mut handles = Vec::new();
    for _i in 0..threads {
        let url_clone = url.clone();
        let handle = tokio::spawn(async {
            contact(url_clone).await;
        });
        handles.push(handle);
    }

    loop {
        let waker = futures::task::noop_waker();
        let mut cx = std::task::Context::from_waker(&waker);
    
        let index = match select_next(&mut handles, &mut cx) {
            Some(index) => index,
            None => { continue;}, // skip the current iteration and go to the next one
        };
        let url_clone = url.clone();
        let handle = tokio::spawn(async {
            contact(url_clone).await;
        });
        handles[index] = handle;
    }
}

fn select_next(handles: &mut Vec<tokio::task::JoinHandle<()>>, cx: &mut std::task::Context<'_>) -> Option<usize> {
    for (i, handle) in handles.iter_mut().enumerate() {
        match handle.poll_unpin(cx) {
            Poll::Ready(_) => {
                // Task completed, return its index
                return Some(i);
            }
            Poll::Pending => {
                // Task still running, continue loop
                continue;
            }
        }
    }
    // All tasks are still running, return None
    None
}