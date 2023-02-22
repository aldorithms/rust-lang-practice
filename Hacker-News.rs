use serde::Deserialize;
use std::sync::{Arc, Mutex};

const STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";
const ITEM_URL_BASE: &str = "https://hacker-news.firebaseio.com/v0/item";

#[derive(Deserialize)]
struct Story
{
    title: String,
}

fn main()
{
    let story_ids: Arc<Vec<u64>> = Arc::new
    (
        request::get
        (
            STORIES_URL
        ).unwrap().json().unwrap()
    );
    let cursor = Arc::new
    (
        Mutex::new(0)
    );
    let mut handles = Vec::new();
    for _ in 0..8
    {
        let cursor = cursor.clone();
        let story_ids = story_ids.clone();
        handles.push
        (
            std::thread::spawn
            (
                move || loop
                {
                    let index = 
                    {
                        let mut cursor_guard = cursor.lock().unwrap();
                        let index = *cursor_guard;
                        if index >= story_ids.len()
                        {
                            return;
                        }
                        *cursor_guard += 1;
                        index
                    };

                    let story_url = format!
                    (
                        "{}/{}.json", 
                        ITEM_URL_BASE, 
                        story_ids
                        [
                            index
                        ]
                    );

                    let story: Story = request::get
                    (
                        &story_url
                    ).unwrap().json().unwrap();
                    
                    println!
                    (
                        "{}", story.title
                    );
                }
            )
        );
    }
    for handle in handles
    {
        handle.join().unwrap();
    }
}