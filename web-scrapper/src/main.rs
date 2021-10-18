use select::document::Document;
use select::predicate::Name;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* Fetch the website with GET Request */
    let body =
        reqwest::blocking::get("https://www.chefkoch.de/rezepte/was-koche-ich-heute/")?.text()?;

    /* Parse the HTML txt, extract the urls and filter the relevant ones */
    let yum = Document::from(body.as_str())
        .select(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|x| x.contains("www.chefkoch.de/rezepte/"))
        .next()
        .unwrap()
        .split("/")
        .last()
        .unwrap()
        .replace(".html", "")
        .replace("-", " ");

    println!("{}", yum);
    Ok(())
}
