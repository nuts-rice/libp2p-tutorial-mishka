

const STORAGE_FILE_PATH: &str = "./recipes.json";
type Result<T> = std::result::Result<T, Box<dyn std::Error + Send + Sync + 'static>>;
static KEYS: Lazy<identify::Keypair> = Lazy::new(|| identify::Keypair::generate_ed25519());
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("recipes"));


fn main() {
    println!("Hello, world!");
}
