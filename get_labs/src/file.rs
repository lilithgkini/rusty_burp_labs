use rand::{seq::IteratorRandom, thread_rng};
use std::path::PathBuf;
use tokio::fs::{read_to_string, write};

pub async fn change_file(file_path: PathBuf) -> anyhow::Result<String> {
    let contents = read_to_string(&file_path).await?;
    let contents_copy = contents.clone();
    let mut rng = thread_rng();
    let choice = contents
        .trim()
        .split("\n")
        .filter(|line| !line.contains("-"))
        .choose(&mut rng)
        .unwrap();
    //println!("{}", choice);

    let content = contents_copy
        .trim()
        .split("\n")
        .map(|line| {
            if line.trim() == choice {
                let s = format!("-{choice}");
                s
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");
    //println!("{}", content);
    write(file_path, content).await?;

    Ok(choice.to_string())
}

pub async fn write_contents(file_path: PathBuf, contents: String) -> anyhow::Result<()> {
    //let file_path: PathBuf = "content.txt".into();
    //let file_path = "content.txt";
    let contents = contents.as_bytes();
    write(file_path, contents).await?;
    Ok(())
}
