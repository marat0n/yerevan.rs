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

[yerevan.rs](https://github.com/marat0n/yerevan.rs) — the computation expressions (do-notations) library
for rust, inspired by F#'s CEs but with syntax-design changes and improvements.

# Docs

Firstly define the struct which is implementing all neccessary functions for <span>yerevan.rs</span>\'s computation expressions.
Do it like this:
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
| `$some_your_struct` => | Defining the struct as the provider of computation expression functions. |
|`let!` $your_var = $your_expression|Using the last defined struct in macros as the provider of `bind` function and call the `$that_struct::bind($your_expression, &\|$your_var\| { next code })` expression.|
|`let` $your_var = $your_expression|Just defining the variable `$your_var`|
|`do!` $your_expression|Using the last defined struct in macros as the provider of `bind` function and call the `$that_struct::bind($your_expression, &\|_\| { next code })` expression.|
|`do` $your_expression|Simply run `$your_expression`|
|`ret!` $your_expression|Using the last defined struct in macros as the provider of `ret_from` function and call the `$that_struct::ret_from($your_expression)` expression.|
|`ret` $your_expression|Using the last defined struct in macros as the provider of `ret` function and call the `$that_struct::ret($your_expression)` expression.|
|`yield` $your_expression|Using the last defined struct in macros as the provider of `combine` and `ret_yield` functions and call the `$that_struct::combine(yer!($that_struct => next code, $that_struct::ret_yield($your_expression))` expression.|



# Examples

soon...


