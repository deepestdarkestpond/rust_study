use std::println as print_line;
fn main() {
    for count in 1u8..15 {
        if count % 3 == 0 {
            print_line!("Count is a multiple of 3: {}", count);
        }
        if count > 10 {
            print_line!("Count exceeded 10!");
        }

        let status = if count > 0 { "active" } else { "idle" };

        print_line!("Count: {}, Status: {}", count, status);
    }
}
