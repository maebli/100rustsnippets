

fn main() {
    let alice_in_wonderland =
        reqwest::blocking::get("https://www.gutenberg.org/files/11/11-0.txt");

    println!(" sentence count for alice in wonderland:{}",
             count_sentences(&alice_in_wonderland.unwrap().text().unwrap()));
}

fn count_sentences(text:&String) -> u32 {
    let mut sentence_count = 0;
    for c in text.chars(){
        match c {
            '!' | '?'|'.'=> sentence_count+=1,
            _ => ()
        }
    }
    sentence_count
}

#[test]
fn test_count_sentences(){
    // source : https://www.gutenberg.org/files/11/11-0.txt
    let txt = String::from("Alice was beginning to get very tired of sitting by
    her sister on the bank, and of having nothing to do: once or twice she
    had peeped into the book her sister was reading, but it had no pictures or
    conversations in it, “and what is the use of a book,” thought Alice
    “without pictures or conversations?”

    So she was considering in her own mind (as well as she could, for the
    hot day made her feel very sleepy and stupid), whether the pleasure of
    making a daisy-chain would be worth the trouble of getting up and
    picking the daisies, when suddenly a White Rabbit with pink eyes ran
    close by her.

    There was nothing so _very_ remarkable in that; nor did Alice think it
    so _very_ much out of the way to hear the Rabbit say to itself, “Oh
    dear! Oh dear! I shall be late!” (when she thought it over afterwards,
    it occurred to her that she ought to have wondered at this, but at the
    time it all seemed quite natural); but when the Rabbit actually _took a
    watch out of its waistcoat-pocket_, and looked at it, and then hurried
    on, Alice started to her feet, for it flashed across her mind that she
    had never before seen a rabbit with either a waistcoat-pocket, or a
    watch to take out of it, and burning with curiosity, she ran across the
    field after it, and fortunately was just in time to see it pop down a
    large rabbit-hole under the hedge.");

    assert_eq!(count_sentences(&txt),6)
}


