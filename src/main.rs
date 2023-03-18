mod utils {
    pub mod attribute;
    pub mod before_runtime;
    pub mod folder_searcher;
    pub mod generator;
    pub mod image_gen;
    pub mod layer;
}
// mod image_gen;
// mod my_gen;
use utils::generator::Generator;
use utils::layer::Layer;
// mod layer;
// mod generator;

//define a static stirng called description
static DESCRIPTION: &'static str = "This is our super cool collection";
static IMAGE_PREFIX: &'static str = "NFT";
static START_TOKEN_ID: u32 = 500;
static END_TOKEN_ID: u32 = 700;

fn main() {
    if START_TOKEN_ID > END_TOKEN_ID {
        panic!("START_TOKEN_ID must be less than END_TOKEN_ID");
    }
    utils::before_runtime::before_runtime();
    let start_time = std::time::Instant::now();
    let layers = vec![
        Layer::new(
            String::from("Background"),
            String::from("layers/Background"),
        ),
        Layer::new(
            String::from("Background Touch"),
            String::from("layers/Background Touch"),
        ),
        Layer::new(String::from("Color Filter A"), String::from("layers/Color Filter A")),
        Layer::new(String::from("Bereichit Base"), String::from("layers/Bereichit Base")),
        Layer::new(String::from("Bereichit Touch A"), String::from("layers/Bereichit Touch A")),
        Layer::new(String::from("Color Filter B"), String::from("layers/Color Filter B")),
        Layer::new(String::from("Element A"), String::from("layers/Element A")),
        Layer::new(String::from("Barbed 1H"), String::from("layers/Barbed 1H")),
        Layer::new(String::from("Barbed 2H"), String::from("layers/Barbed 2H")),
        Layer::new(String::from("Barbed 3H"), String::from("layers/Barbed 3H")),
        Layer::new(String::from("Barbed 4H"), String::from("layers/Barbed 4H")),
        Layer::new(String::from("Barbed 5H"), String::from("layers/Barbed 5H")),
        Layer::new(String::from("Barbed 6H"), String::from("layers/Barbed 6H")),
        Layer::new(String::from("Barbed 7H"), String::from("layers/Barbed 7H")),
        Layer::new(String::from("Barbed 8H"), String::from("layers/Barbed 8H")),
        Layer::new(String::from("Barbed 9H"), String::from("layers/Barbed 9H")),
        Layer::new(String::from("Barbed 10H"), String::from("layers/Barbed 10H")),
        Layer::new(String::from("Barbed 11H"), String::from("layers/Barbed 11H")),
        Layer::new(String::from("Barbed 12H"), String::from("layers/Barbed 12H")),
        Layer::new(String::from("Element B"), String::from("layers/Element B")),
        Layer::new(String::from("Dot"), String::from("layers/Dot")),
        Layer::new(String::from("Signature"), String::from("layers/Signature")),
    ];

    let my_gen = Generator::new(
        START_TOKEN_ID,
        END_TOKEN_ID,
        layers,
        DESCRIPTION,
        IMAGE_PREFIX,
    );
    my_gen.run_generation();

    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!(
        "Time taken to generate {} images: {:?}",
        END_TOKEN_ID - START_TOKEN_ID,
        duration
    );
}
