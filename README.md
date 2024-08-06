```
                                                  
                       __| __|                    
                      |  \__ \                    
                   _)_|  ____/       //~\         
                                   //  ~~\\       
              _                 /───   ~~  \\     
             / \              ───      ~~  ~~\    
            / ~~\\          /           ~~~   \   
          //   ~~\\\ ~\   /            ~~~   ~~\\ 
        //     ~~~ \~─~──── ~~         ~~~~~      
   ~──//         ~~~                              
───────╮  ~~~~   ~     ~ ~ ~~~~~~~ ~~~~~~~~~~~~~  
 #  #  │╮    ~~~~~~~~~~~~│  '│╭──── ╭─  ╭^^^^^^^^^
       │╯         │`  │  ╰╮───╯  ╭──╯   │ @ @ @ @ 
 #  #╭╮│╮            ╭╯   │  ╭───╯  ╭───┼─────────
   ╭─╰╯──^^╮    │`   │.   │──╯      │             
 ╭─╰───────╯^╮───────│    │  '│   ╭─│ #  #  #  #  
────────╮@ @ │──╮    │  ..│       ╰─│             
        │    │  ╰───╭╯   ╭╯         │             
 # # # #│@ @ │      │    ╰─╮ '│   ╭─│ #  #  #  #  
        │────╯ '│`  │  .   ╰╮ │   ╰─│             
 # # # #│       │   │       │       │             
        │          ╭╯    .  ╰╮    ╭─│ #  #  #  #  
 # # # #│  '│`  ╭──╯  .      ╰╮   ╰─│             
        │   │   │     .       │     │             
```


# Overview

[yerevan.rs](https://github.com/marat0n/yerevan.rs) is a computation expressions (do-notations) library for Rust,
inspired by F#'s CEs, but with changes and improvements in syntax-design.

# Docs

Firstly define the struct which is implementing all the neccessary functions for <span>yerevan.rs</span>\'s computation expressions.
Like this:
```rust
yer! {
    MyOption =>
    let! ...
    let ...
    ret ...
}
```

|yer! expression|description|
| --- | ---|
| `$your_struct` => | Defines the struct as the provider of computation expression functions. |
|`let!` $your_var = $your_expression|Uses the last defined struct in macros as the provider of `bind` function and calls the `$that_struct::bind($your_expression, &\|$your_var\| { next code })` expression.|
|`let` $your_var = $your_expression|Defines the variable `$your_var`|
|`do!` $your_expression|Uses the last defined struct in macros as the provider of `bind` function and calls the `$that_struct::bind($your_expression, &\|_\| { next code })` expression.|
|`do` $your_expression|Simply runs `$your_expression`|
|`ret!` $your_expression|Uses the last defined struct in macros as the provider of `ret_from` function and calls the `$that_struct::ret_from($your_expression)` expression.|
|`ret` $your_expression|Uses the last defined struct in macros as the provider of `ret` function and calls the `$that_struct::ret($your_expression)` expression.|
|`yield` $your_expression|Uses the last defined struct in macros as the provider of `combine` and `ret_yield` functions and calls the `$that_struct::combine(yer!($that_struct => next code, $that_struct::ret_yield($your_expression))` expression.|



# Examples

soon...


