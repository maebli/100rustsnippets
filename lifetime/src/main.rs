
// Boilerplate for my singelton example
struct A;
static THING:A = A{};
fn getInstance() -> &'static A {
    &THING
}

fn main() {

    {
        // The following two lines are identical
        // due to static lifetime elision both
        // variables have a 'static lifetime
        const X1: &str = "a";
        const X2: &'static str = "a";

        // The following will add three string literals
        // to the memory of the binary
        let _a = X1;
        let _b = X1;
        let _c = X1;

        // When  these constants go out of scope, their references
        // can no longer be used, but the data remains in the binary.
    }

    {
        // Two ways to make a constant with `'static` lifetime.
        let X1: &'static str = "a";
        static X2: &str = "a";

        // The following will add three references
        // non mutable borrows to X1 and NOT three
        // three new string literals to the binary
        let _a = X1;
        let _b = X1;
        let _c = X1;
        // When  these constants go out of scope, their references
        // can no longer be used, but the data remains in the binary.
    }


    {
        // Implementing the singleton pattern
        let _a = getInstance();
        let _b = getInstance();
    }

}
