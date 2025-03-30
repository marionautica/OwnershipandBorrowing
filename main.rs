fn main() {
    println!("========================================");
    println!("RUST OWNERSHIP AND BORROWING DEMO");
    println!("========================================");
    println!("This program demonstrates Rust's ownership and borrowing concepts");
    println!("through a series of practical examples.\n");

    // Section 1: Basic Ownership
    println!("SECTION 1: BASIC OWNERSHIP");
    println!("------------------------------------------");
    
    println!("Example 1: Transfer of ownership");
    // When we assign a variable to another variable, the ownership is transferred
    {
        let s1 = String::from("hello"); // s1 is the owner of this String
        println!("  Created s1: {}", s1);
        
        let s2 = s1; // ownership moves from s1 to s2
        println!("  Ownership transferred to s2: {}", s2);
        
        // This would cause a compile error because s1 no longer owns the String
        // println!("  Trying to use s1: {}", s1);
        println!("  Note: We can't use s1 anymore as it no longer owns the String");
    }
    println!("  Variables s1 and s2 are now out of scope, memory is automatically freed\n");

    println!("Example 2: Ownership with functions");
    {
        let s = String::from("hello world");
        println!("  Created string s: {}", s);
        
        // When we pass s to the function, ownership is transferred
        takes_ownership(s);
        
        // This would cause a compile error
        // println!("  Trying to use s: {}", s);
        println!("  Note: We can't use s anymore as its ownership was transferred to the function\n");
        
        let x = 5;
        println!("  Created integer x: {}", x);
        
        // Integers are Copy types, so a copy is made instead of transferring ownership
        makes_copy(x);
        
        println!("  We can still use x after passing it to a function: {}", x);
        println!("  Note: This is because primitive types like integers implement the Copy trait");
    }
    println!();
    
    println!("Example 3: Returning ownership");
    {
        let s1 = gives_ownership();
        println!("  Received ownership of string: {}", s1);
        
        let s2 = String::from("hello");
        println!("  Created s2: {}", s2);
        
        let s3 = takes_and_gives_back(s2);
        println!("  Transferred s2 to function and received it back as s3: {}", s3);
        
        // s2 is no longer valid here
        println!("  Note: s2 is no longer valid as ownership was transferred");
    }
    println!("  s1 and s3 go out of scope and are dropped, freeing memory\n");

    // Section 2: References and Borrowing
    println!("SECTION 2: REFERENCES AND BORROWING");
    println!("------------------------------------------");
    
    println!("Example 1: Immutable references (borrowing)");
    {
        let s1 = String::from("hello");
        println!("  Created string s1: {}", s1);
        
        // Here, calculate_length borrows s1 but doesn't take ownership
        let len = calculate_length(&s1);
        
        println!("  Length of '{}' is {} characters", s1, len);
        println!("  Note: We can still use s1 here because we only passed a reference to the function");
    }
    println!();
    
    println!("Example 2: Mutable references");
    {
        let mut s = String::from("hello");
        println!("  Created mutable string s: {}", s);
        
        // Here we pass a mutable reference
        change(&mut s);
        
        println!("  After change, s is now: {}", s);
        println!("  Note: We were able to modify s through a mutable reference");
        
        println!("\n  Restriction: Only one mutable reference to a particular piece of data in a particular scope");
        
        {
            let mut s = String::from("multiple");
            println!("  Created mutable string s: {}", s);
            
            let r1 = &mut s;
            println!("  Created mutable reference r1 to s");
            
            // This would cause a compile error
            // let r2 = &mut s;
            
            println!("  Can't create another mutable reference while r1 exists");
            println!("  Using r1: {}", r1);
        } // r1 goes out of scope here, so we can create a new mutable reference to s
        
        println!("\n  Restriction: Cannot have mutable and immutable references in the same scope");
        
        {
            let mut s = String::from("hello");
            println!("  Created mutable string s: {}", s);
            
            let r1 = &s; // immutable reference
            let r2 = &s; // another immutable reference - this is fine
            
            println!("  Two immutable references: {} and {}", r1, r2);
            
            // This would cause a compile error
            // let r3 = &mut s;
            
            println!("  Can't create a mutable reference while immutable references exist");
        }
    }
    println!();
    
    // Section 3: The Slice Type
    println!("SECTION 3: SLICES");
    println!("------------------------------------------");
    
    println!("Example: String slices");
    {
        let s = String::from("hello world");
        println!("  Created string s: {}", s);
        
        let hello = &s[0..5];
        let world = &s[6..11];
        
        println!("  Created slices: '{}' and '{}'", hello, world);
        println!("  Note: Slices are references to a portion of the String");
        println!("  This means they don't take ownership of the data");
    }
    println!();
    
    // Section 4: Practical Example
    println!("SECTION 4: PRACTICAL EXAMPLE");
    println!("------------------------------------------");
    
    {
        let text = String::from("The quick brown fox jumps over the lazy dog");
        println!("  Original text: {}", text);
        
        let first_word = get_first_word(&text);
        println!("  First word: {}", first_word);
        
        // Demonstrate how borrowing prevents modification
        println!("\n  Demonstrating how borrowing prevents data races:");
        
        let mut mutable_text = String::from("Hello world");
        println!("  Created mutable text: {}", mutable_text);
        
        // Get a reference to the first word
        let word = get_first_word(&mutable_text);
        println!("  First word reference: {}", word);
        
        // This would cause a compile error because we have an immutable reference
        // while trying to make a mutable one
        // mutable_text.clear();
        
        println!("  Can't modify mutable_text while word reference exists");
        println!("  This prevents a data race where word would be pointing to invalid memory");
        
        println!("  Using word: {}", word); // Using word
        
        // Now we can modify mutable_text because word is no longer used
        mutable_text.clear();
        println!("  After word is no longer used, we can modify text: '{}'", mutable_text);
    }
    
    // Summary
    println!("\n========================================");
    println!("SUMMARY");
    println!("========================================");
    println!("1. Each value in Rust has a single owner.");
    println!("2. When the owner goes out of scope, the value is dropped.");
    println!("3. You can transfer ownership by assigning or passing a value.");
    println!("4. References allow you to access a value without taking ownership.");
    println!("5. Immutable references (&T) allow reading but not modification.");
    println!("6. Mutable references (&mut T) allow modification but come with restrictions:");
    println!("   - Only one mutable reference at a time");
    println!("   - Cannot have mutable and immutable references simultaneously");
    println!("7. Slices are references to portions of collections.");
    println!("8. Rust's ownership system prevents memory safety issues at compile time.");
}

// This function takes ownership of the String passed to it
fn takes_ownership(some_string: String) {
    println!("  Function received ownership of: {}", some_string);
} // some_string goes out of scope and `drop` is called, freeing memory

// This function takes a copy of the value passed to it
fn makes_copy(some_integer: i32) {
    println!("  Function received a copy of: {}", some_integer);
} // some_integer goes out of scope but nothing special happens

// This function creates and returns a String, transferring ownership to the caller
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    println!("  Function created a string: {}", some_string);
    some_string // Return and transfer ownership
}

// This function takes and returns ownership of a String
fn takes_and_gives_back(a_string: String) -> String {
    println!("  Function received ownership of: {}", a_string);
    a_string // Return and transfer ownership back
}

// This function borrows a String but doesn't take ownership
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // s goes out of scope, but it doesn't have ownership, so nothing is dropped

// This function takes a mutable reference and modifies the value
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// This function takes a string slice and returns the first word
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
