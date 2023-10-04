#![allow(non_snake_case)]
use kdam::{
    rayon::prelude::*, term, tqdm, BarExt, Column, RichProgress, Spinner, TqdmParallelIterator,
};
use once_cell::sync::Lazy;
use regex::{Regex, RegexSet};
use std::fs::File;
use std::io::{prelude::*, stderr, BufReader, BufWriter, IsTerminal, Result, Write};
fn main() {
    term::init(stderr().is_terminal());
    term::hide_cursor().unwrap();
    let mut linecount = mapping("./mapping.jsonl").unwrap();
    deep_search("./mapping.jsonl", "./dump.jsonl").unwrap();
    linecount = filter("./dump.jsonl", "./filtered-output.jsonl", linecount).unwrap();
    write_dict("./filtered-output.jsonl", "dict/content.html", linecount);
}

fn mapping(output_path: &str) -> Result<usize> {
    let api_endpoint: &str = "https://wh40k.lexicanum.com/mediawiki/api.php";
    let params: Vec<(String, String)> = vec![
        ("action".to_string(), "query".to_string()),
        ("format".to_string(), "json".to_string()),
        ("prop".to_string(), "info".to_string()),
        ("rawcontinue".to_string(), "1".to_string()),
        ("generator".to_string(), "allpages".to_string()),
        ("inprop".to_string(), "url".to_string()),
        ("gapfilterredir".to_string(), "nonredirects".to_string()),
        ("gaplimit".to_string(), "500".to_string()),
    ];
    let mut params_gap: Vec<(String, String)> = vec![("gapcontinue".to_string(), "".to_string())];
    let output_file: File = File::create(output_path)?;

    let mut output_buffer: BufWriter<File> = BufWriter::new(output_file);

    let mut pb = RichProgress::new(
        tqdm!(),
        vec![
            Column::Spinner(Spinner::new(
                &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
                30.0,
                1.0,
            )),
            Column::Text("[bold blue]Mapping...".to_owned()),
            Column::Animation,
            Column::Percentage(1),
            Column::Text("•".to_owned()),
            Column::CountTotal,
            Column::Text("•".to_owned()),
            Column::Rate,
        ],
    );
    let mut linecount: usize = 0;
    loop {
        // Exploratory search
        let new_params: Vec<(String, String)> =
            params.iter().chain(params_gap.iter()).cloned().collect();
        let url: reqwest::Url = reqwest::Url::parse_with_params(api_endpoint, &new_params).unwrap();
        let client: reqwest::blocking::Client =
            reqwest::blocking::ClientBuilder::new().build().unwrap();
        let body: String = client.get(url).send().unwrap().text().unwrap();
        let data: json::JsonValue = json::parse(&body).unwrap();

        let pages: Vec<&json::JsonValue> = data["query"]["pages"]
            .entries()
            .map(|(_, v)| v)
            .collect::<Vec<_>>();

        for (idx, page) in pages.iter().enumerate() {
            // for every 20th index, do the thing, but the remainder is also useful
            if idx % 20 == 0 {
                let mut output: json::JsonValue = json::JsonValue::new_object();
                output["title"] = page["title"].clone();
                output_buffer.write_all(&output.to_string().as_bytes())?;
                output_buffer.write_all("\n".as_bytes())?;
            }
            linecount += 1;
        }
        let new_data: String = data["query-continue"]["allpages"]["gapcontinue"]
            .to_string()
            .clone();
        if data["query-continue"]["allpages"].has_key("gapcontinue") {
            params_gap = vec![("gapcontinue".to_string(), new_data.to_string())];
        } else {
            break;
        }
        pb.update(1)?;
    }
    eprintln!();
    eprintln!("Exploratory search complete!");
    Ok(linecount)
}

fn deep_search(input_path: &str, output_path: &str) -> Result<()> {
    let api_endpoint: &str = "https://wh40k.lexicanum.com/mediawiki/api.php";
    let params: Vec<(String, String)> = vec![
        ("action".to_string(), "query".to_string()),
        ("format".to_string(), "json".to_string()),
        ("prop".to_string(), "redirects|info|extracts".to_string()),
        ("rawcontinue".to_string(), "1".to_string()),
        ("generator".to_string(), "allpages".to_string()),
        ("rdprop".to_string(), "title|pageid".to_string()),
        ("rdlimit".to_string(), "max".to_string()),
        ("inprop".to_string(), "url".to_string()),
        ("exlimit".to_string(), "max".to_string()),
        ("exintro".to_string(), "1".to_string()),
        ("explaintext".to_string(), "1".to_string()),
        ("gapfilterredir".to_string(), "nonredirects".to_string()),
        ("gaplimit".to_string(), "20".to_string()),
    ];

    let input_file: File = File::open(input_path)?;
    let input_buffer: BufReader<File> = BufReader::new(input_file);

    eprintln!("Starting deep search...");
    let mut tasks: Vec<String> = vec![];
    for line in input_buffer.lines() {
        tasks.push(line.unwrap());
    }
    let client = reqwest::blocking::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(30))
        .pool_idle_timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap();
    tasks.into_par_iter().tqdm().for_each(move |task| {
        let output_file: File = File::options()
            .append(true)
            .create(true)
            .open(output_path)
            .unwrap();
        let mut output_buffer: BufWriter<File> = BufWriter::new(output_file);
        let params_gap: Vec<(String, String)> = vec![(
            "gapcontinue".to_string(),
            json::parse(&task).unwrap()["title"].to_string(),
        )];
        do_search(
            &client.clone(),
            &params,
            &params_gap,
            api_endpoint,
            &mut output_buffer,
        )
        .unwrap();
    });
    Ok(())
}

fn do_search(
    client: &reqwest::blocking::Client,
    params: &Vec<(String, String)>,
    params_gap: &Vec<(String, String)>,
    api_endpoint: &str,
    output_buffer: &mut BufWriter<File>,
) -> Result<()> {
    let new_params: Vec<(String, String)> =
        params.iter().chain(params_gap.iter()).cloned().collect();
    let url: reqwest::Url = reqwest::Url::parse_with_params(api_endpoint, &new_params).unwrap();
    let body = client.get(url).send().unwrap().text().unwrap();
    let data: json::JsonValue = json::parse(&body).unwrap();
    let pages: Vec<&json::JsonValue> = data["query"]["pages"]
        .entries()
        .map(|(_, v)| v)
        .collect::<Vec<_>>();
    for page in pages {
        let mut output: json::JsonValue = json::JsonValue::new_object();
        output["title"] = page["title"].clone();
        output["pageid"] = page["pageid"].clone();
        output["extract"] = page["extract"].clone();
        if page.has_key("redirects") {
            output["redirects"] = page["redirects"].clone();
        }
        output_buffer.write_all(&output.to_string().as_bytes())?;
        output_buffer.write_all("\n".as_bytes())?;
    }
    Ok(())
}

fn filter(input_path: &str, output_path: &str, linecount: usize) -> Result<usize> {
    let input_file: File = File::open(input_path)?;
    let input_buffer: BufReader<File> = BufReader::new(input_file);
    let mut output_file: File = File::create(output_path)?;

    //regex patterns
    let filters: RegexSet = RegexSet::new(&[
        r"(?i)\((disambiguation|song|novel|anthology|novella|list|game|short story|novel series|audio)\)",
        r"(?i)\b(miniatures|magazine|white dwarf|catalog)\b",
        r"(?i)\([0-9]{4}\)",
        r"(?i)(rulebook|game)"
    ])
    .unwrap();
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[.*?\]").unwrap());
    eprintln!();
    let mut pb = RichProgress::new(
        tqdm!(total = linecount),
        vec![
            Column::Spinner(Spinner::new(
                &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
                30.0,
                1.0,
            )),
            Column::Text("[bold blue]Filtering...".to_owned()),
            Column::Animation,
            Column::Percentage(1),
            Column::Text("•".to_owned()),
            Column::CountTotal,
            Column::Text("•".to_owned()),
            Column::Rate,
            Column::Text("•".to_owned()),
            Column::RemainingTime,
        ],
    );
    let mut linecount: usize = 0;
    for line in input_buffer.lines() {
        pb.update(1)?;
        linecount += 1;
        let data: json::JsonValue = json::parse(&line.unwrap()).unwrap();
        let data_title: String = data["title"].to_string();
        let mut data_extract: String = data["extract"].to_string();
        if data_extract.contains("[") {
            data_extract = RE.replace_all(&data_extract.to_string(), "").to_string();
        };
        if data_extract.is_empty() {
            continue;
        };
        if filters.is_match(&data_title.to_string()) {
            continue;
        };
        output_file.write_all(&data.to_string().as_bytes())?;
        output_file.write_all("\n".as_bytes())?;
    }
    Ok(linecount)
}

fn write_dict(input_path: &str, output_path: &str, linecount: usize) {
    let input_file: File = File::open(input_path).unwrap();
    let input_buffer: BufReader<File> = BufReader::new(input_file);

    let output_file: File = File::options()
        .append(true)
        .create(true)
        .open(output_path)
        .unwrap();
    let mut output_buffer: BufWriter<File> = BufWriter::new(output_file);

    output_buffer
        .write_all(
            "<html xmlns:math=\"http://exslt.org/math\" mlns:svg=\"http://www.w3.org/2000/svg\" 
    xmlns:tl=\"https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf\" 
    xmlns:saxon=\"http://saxon.sf.net/\" xmlns:xs=\"http://www.w3.org/2001/XMLSchema\" 
    xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" \
    xmlns:cx=\"https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf\" 
    xmlns:dc=\"http://purl.org/dc/elements/1.1/\" 
    xmlns:mbp=\"https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf\" 
    xmlns:mmc=\"https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf\" 
    xmlns:idx=\"https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf\"
    <meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\">
        <style>
            h5 {
                font-size: 1em;
                margin: 0;
            }
            dt {
                font-weight: bold;
            }
            dd {
                margin: 0;
                padding: 0 0 0.5em 0;
                display: block
            }
        </style>
    </head>
    <body>
        <mbp:frameset>"
                .to_string()
                .as_bytes(),
        )
        .unwrap();

    let mut pb = RichProgress::new(
        tqdm!(total = linecount),
        vec![
            Column::Spinner(Spinner::new(
                &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
                30.0,
                1.0,
            )),
            Column::Text("[bold blue]Writing HTML...".to_owned()),
            Column::Animation,
            Column::Percentage(1),
            Column::Text("•".to_owned()),
            Column::CountTotal,
            Column::Text("•".to_owned()),
            Column::Rate,
            Column::Text("•".to_owned()),
            Column::RemainingTime,
        ],
    );

    for line in input_buffer.lines() {
        pb.update(1).unwrap();
        let data: json::JsonValue = json::parse(&line.unwrap()).unwrap();
        let mut infls: Vec<String> = vec![];

        if data.has_key("redirects") {
            for redirect in data["redirects"].members() {
                infls.push(redirect["title"].to_string());
            }
        };

        output_buffer
            .write_all(
                format!(
                    "
        <idx:entry name=\"default\" scriptable=\"yes\" spell=\"yes\">
            <h5>
                <dt>
                    <idx:orth> {}",
                    data["title"]
                )
                .to_string()
                .as_bytes(),
            )
            .unwrap();

        if infls.len() > 0 {
            output_buffer
                .write_all("\n<idx:infl>".to_string().as_bytes())
                .unwrap();
            for infl in infls {
                output_buffer
                    .write_all(
                        format!("\n<idx:iform value=\"{}\"", infl)
                            .to_string()
                            .as_bytes(),
                    )
                    .unwrap();
            }
            output_buffer
                .write_all("\n<idx:infl>".to_string().as_bytes())
                .unwrap();
        }
        output_buffer
            .write_all(
                format!(
                    "
                    </idx:orth>
                </dt>
            </h5>
            <dd>{}</dd>
        </idx:entry>
        <hr />",
                    data["extract"]
                )
                .to_string()
                .as_bytes(),
            )
            .unwrap();
    }

    output_buffer
        .write_all(
            "</mbp:frameset>
            </body>"
                .to_string()
                .as_bytes(),
        )
        .unwrap();
}
