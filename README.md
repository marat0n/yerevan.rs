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
yerevanize! {
    MyOption =>
    let! ...
    let ...
    ret ...
}
```

|yerevanize! expression|description|
| --- | ---|
| `$some_your_struct` => | Defining the struct as the provider of computation expression functions. |
|`let!` $your_var = $your_expression|Using the last defined struct in macros as the provider of `bind` function and call the `$that_struct::bind($your_expression, &\|$your_var\| { next code })`. |



# Examples

soon...


