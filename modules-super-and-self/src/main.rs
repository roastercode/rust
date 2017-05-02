/* Aurélien DESBRIÈRES
aurelien(at)hackers(dot)camp
License GNU GPL latest */

// Rust experimentations
// Modules super and self in Rust

fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // Let's access all the functions name `function` from this scope!
        print!("called `my::indirect_call()`, that\n> ");

        // The `self` keyword refers to the current module scope - in this case `my`.
        // Calling `self::function()` and calling `funciton()` directly both give
        // the same result, because they reer to the same function.
        self::function();
        function();

        // We can also use `self` to access another module inside `my`:
        self::cool::function();

        // The `super` keyword refers to the parent scope (outside the `my` module).
        super::function();
        
        // This will bind to the `cool::function` in the *crate* scope.
        // In this case th ecrate scope is the outermost scope.
        {
            use cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}

