use unicode_segmentation::UnicodeSegmentation;

fn main() {
    /*
    let mut s: String = String::from("foo");
    s.push_str("bar");
    s.push('!');

    println!("{}", s);

    let s1: String = String::from("Hello ");
    let s2: String = String::from("world");
    let s3: String = s1 + &s2; // Taking ownership from s1 and only taking reference from s2
                               // This approach saves memory

    println!("{}", s3);

    //? using format macro
    let s1: String = String::from("Hello");
    let s2: String = String::from("world");
    let s3: String = format!("{} {} \n", s1, s2); // This approach saves memory
    println!("{}", s3);

    //? iterating over String
    let s: String = String::from("hello world");
    for c in s.chars() {
        println!("{}", c);
    }
    */
    //*Bytes and Scalar Values and Grapheme Clusters */
    bytes_scalar_grapheme();
}

fn bytes_scalar_grapheme() {
    let namaste: String = String::from("नमस्ते");

    //? bytes */
    for b in namaste.bytes() {
        println!("{}", b);
    }
    //* outputs
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    //* Scalar */
    for s in namaste.chars() {
        println!("{} ", s);
    }
    //* outputs */
    // ['न', 'म', 'स', '्', 'त', 'े']

    //* Grapheme clusters */
    //* we need `unicode-segmentation` crate for this */
    for g in namaste.graphemes(true) {
        println!("{}", g);
    }

    //* outputs */
    // ["न", "म", "स्", "ते"]
}
