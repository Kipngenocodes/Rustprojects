fn main() {
    // Constants in Rust are declared with `const` (always immutable)
    // They must have an explicit type and are evaluated at compile time
    const MAX_HEALTH: i32 = 100;
    const PI: f64 = 3.14159265359;
    const APP_VERSION: &str = "1.0.0";
    const IS_PRODUCTION: bool = true;

    // You can use underscores for readability in numbers
    const ONE_MILLION: u32 = 1_000_000;

    println!("App version: {}", APP_VERSION);
    println!("Max health: {}", MAX_HEALTH);
    println!("Pi ≈ {}", PI);
    println!("One million: {}", ONE_MILLION);
    println!("Production mode: {}", IS_PRODUCTION);

    // This would NOT compile (constants can't be mutated):
    // MAX_HEALTH = 200;
}