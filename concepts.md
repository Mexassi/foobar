# Variables
Declare variables using `let`. All variables are immutable by default. They can be made mutable using `mut`.
```rs
let x = 42;
let mut y = 42;
```
## Shadowing
Shadowing allows you to reuse the same variable name.
```rs
let x = 41;
let x = 42;
```
In this example, the second `let` statement shadows the first one, so `x` is now `42`. Shadowing can be used multiple times and it is scope bound. So the shadowing only applies within the scope it is defined in.

# Data Types
## Scalar types
A scalar represents a single value, there are four main scalar types: integers, floating-point, numbers, Booleans and characters.

### Integer types
Integer can be signed or unsinged and from sizes to 8-bit and 128-bit to isize/usize which is architecture-dependent. (i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize)

### Number literals
Decimal, Hex, Octal, Binary, Byte

It's important to not that when storing a value of 256 on a u8 which only goes up to 255, the value will wrap around to 0 when compiling wiht --release, however when compiling in debug mode the value will overflow and panic at runtime.

## Compound types
Rust has 2 types of compound types: arrays and tuples.

# Functions
are straightforward and use `fn` to define them. Call them by name followed by parentheses and any arguments.
A function can return a value in rust to sign a function return type use `->` followed by the type. Then in the function body the return value is the value of the final expression.

# Flow control
Controlling the flow is really simple in Rust using the if else statement. It works like any other language. It can also be used as a statement to assign a value to a variable.
```rs
let condition = true;
let number = if condition { 42 } else { 0 };
```

# Ownership
Ownership is a unique concept in Rust that allows the compiler to track who owns a value and when it should be cleaned up. 
Traditional languages don't often force you to think about the stack and the heap, however in Rust you do.
The stack is an organized LIFO (Last In, First Out) data structure that stores values on the call stack. To store data in the stack the size of the data must be known at compile time.
The heap is less organized and is used by the allocator to find free memory to store values and return then pointers.
When a function is called, the arguments and the local variables are pushed onto the stack and popped off when the function returns.
Accessing the stack is faster than accessing the heap and rust works faster when working with values that are closed together (on the stack).

### Rules
- each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value is dropped

### String type
Use the String type which is more complex to show the rules of ownership.
Basically a hardcoded string value is always stored on the stack because the size is known at compile time. When a sting is created using String::from, the string is stored on the heap.
The concept of ownership is when a value is owned by a scope and this is passed to a function the function then takes ownership of the value and unless it is returned, the value will be dropped when the function returns.
To pass the string as a reference we use the &str which is a string slice that points to a string stored on the heap. The function can now refer to the value without taking ownership. The action of passing a reference is called borrowing.
A borrowed value cannot be modified because it does not own the value. To fix this we can use a mutable reference to the value but with a restriction that the reference cannot be borrowed as immutable and mutable at the same time.
